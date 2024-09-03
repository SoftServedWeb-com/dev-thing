"use client"
import React, { createContext, useState, useEffect, useContext, ReactNode, Suspense } from "react";
import { invoke } from '@tauri-apps/api/tauri';
import { useSearchParams } from 'next/navigation';
import { listen } from '@tauri-apps/api/event';
import { os } from "@tauri-apps/api";
import { useProjects } from "./useProject";

interface ProjectInfo {
  framework: string;
  runtime: string;
  packages: { name: string; version: string }[];
}

interface ProjectAnalyzerContextType {
  projectName: string;
  projectInfo: ProjectInfo | null;
  error: string | null;
  pid: number | null;
  isRunning: boolean;
  terminalOutput: string;
  setIsRunning: (isRunning: boolean) => void;
  setPid: (pid: number | null) => void;
  setTerminalOutput: (output: string) => void;
  appendTerminalOutput: (output: string) => void;
  resetTerminalOutput: () => void;
  platform: string | null;
}

const ProjectAnalyzerContext = createContext<ProjectAnalyzerContextType | undefined>(undefined);

export const ProjectAnalyzerProvider = ({ children }: { children: ReactNode }) => {
  const { platform } = useProjects();
  const searchParams = useSearchParams();
  const projectNameFromURL = searchParams.get('page') || '';
  const [projectName, setProjectName] = useState<string>(projectNameFromURL);
  const [projectInfo, setProjectInfo] = useState<ProjectInfo | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [pid, setPid] = useState<number | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  const [terminalOutput, setTerminalOutput] = useState('');

  const appendTerminalOutput = (output: string) => {
    setTerminalOutput(prev => {
      const newOutput = prev + output + '\n';
      localStorage.setItem(`${projectName}_terminalOutput`, newOutput);
      return newOutput;
    });
  };

  const resetTerminalOutput = () => {
    setTerminalOutput('');
    localStorage.removeItem(`${projectName}_terminalOutput`);
  };

  useEffect(() => {
    const analyzeProject = async () => {
      console.log("Analyzing project:", projectNameFromURL);
      setProjectName(projectNameFromURL);

      if (projectNameFromURL) {
        const allProjectPath = localStorage.getItem('projectsPath');
        const projectData = localStorage.getItem(projectNameFromURL);
        const storedTerminalOutput = localStorage.getItem(`${projectNameFromURL}_terminalOutput`);

        if (allProjectPath) {
          let projectPath;
          
          if (platform === 'win32') {
            projectPath = `${allProjectPath}\\${projectNameFromURL}`;
          } else {
            console.log("Platform", platform)
            projectPath = `${allProjectPath}/${projectNameFromURL}`;
          }
          console.log("Projects Path", projectPath)
          try {
            const result = await invoke<ProjectInfo>('analyze_project', { path: projectPath });
            setProjectInfo(result);
            setError(null);

            if (projectData) {
              const { pid, isRunner } = JSON.parse(projectData);
              setPid(pid);
              setIsRunning(isRunner);
              console.log(`Project ${projectNameFromURL} - PID: ${pid}, isRunning: ${isRunner}`);
              
              // Set stored terminal output or set it to empty if not running
              setTerminalOutput(isRunner ? (storedTerminalOutput || '') : '');
            } else {
              console.log(`No stored data for project ${projectNameFromURL}`);
              setPid(null);
              setIsRunning(false);
              setTerminalOutput('');
            }

          } catch (err) {
            console.error("Error analyzing project:", err);
            setError("Failed to analyze project. Please check the project path.");
            setProjectInfo(null);
            setPid(null);
            setIsRunning(false);
            setTerminalOutput('');
          }
        } else {
          setError("Projects path not found in localStorage.");
        }
      } else {
        console.log("Project name not set in URL");
        setError("Project name not set. Please provide a valid project name in the URL.");
        setPid(null);
        setIsRunning(false);
        setTerminalOutput('');
      }
    };

    analyzeProject();

    // Set up event listener for project output
    const unlisten = listen('project-output', (event: { payload: [number, string] }) => {
      const [eventPid, output] = event.payload;
      if (eventPid === pid) {
        appendTerminalOutput(output);
      }
    });

    return () => {
      unlisten.then(f => f());
    };
  }, [projectNameFromURL, pid]);

  const contextValue: ProjectAnalyzerContextType = {
    projectName,
    projectInfo,
    error,
    pid,
    isRunning,
    terminalOutput,
    setIsRunning,
    setPid,
    setTerminalOutput,
    appendTerminalOutput,
    resetTerminalOutput,
    platform
  };

  return (
    <ProjectAnalyzerContext.Provider value={contextValue}>
      {children}
    </ProjectAnalyzerContext.Provider>
  );
};

export const useProjectAnalyzer = () => {
  const context = useContext(ProjectAnalyzerContext);
  if (context === undefined) {
    throw new Error('useProjectAnalyzer must be used within a ProjectAnalyzerProvider');
  }
  return context;
};