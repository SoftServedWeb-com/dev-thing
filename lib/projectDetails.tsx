"use client"
import React, { createContext, useState, useEffect, useContext, ReactNode, Suspense } from "react";
import { invoke } from '@tauri-apps/api/tauri';
import { useSearchParams } from 'next/navigation';

interface ProjectInfo {
  framework: string;
  runtime: string;
  packages: { name: string; version: string }[];
}

interface ProjectAnalyzerContextType {
  projectName: string;
  projectInfo: ProjectInfo | null;
  error: string | null;
}

const ProjectAnalyzerContext = createContext<ProjectAnalyzerContextType | undefined>(undefined);

export const ProjectAnalyzerProvider = ({ children }: { children: ReactNode }) => {
  const searchParams = useSearchParams();
  const projectNameFromURL = searchParams.get('page') || '';
 
  const [projectName, setProjectName] = useState(projectNameFromURL);
  const [projectInfo, setProjectInfo] = useState<ProjectInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const analyzeProject = async () => {
      console.log("Analyzing project...");
      if (projectNameFromURL) {
        const allProjectPath = localStorage.getItem('projectsPath');
        if (allProjectPath) {
          const projectPath = `${allProjectPath}/${projectNameFromURL}`;
          try {
            console.log("Invoking analyze_project with path:", projectPath);
            const result = await invoke<ProjectInfo>('analyze_project', { path: projectPath });
            console.log("Analysis result:", result);
            setProjectInfo(result);
            setError(null);
          } catch (err) {
            console.error("Error analyzing project:", err);
            setError("Failed to analyze project. Please check the project path.");
            setProjectInfo(null);
          }
        } else {
          setError("Projects path not found in localStorage.");
        }
      } else {
        console.log("Project name not set in URL");
        setError("Project name not set. Please provide a valid project name in the URL.");
      }
    };

    setProjectName(projectNameFromURL);
    analyzeProject();
  }, [projectNameFromURL]);

  return (
    <ProjectAnalyzerContext.Provider 
      value={{ 
        projectName, 
        projectInfo, 
        error
      }}
    >
    
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

export default ProjectAnalyzerContext;