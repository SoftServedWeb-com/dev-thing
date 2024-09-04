"use client"
import React, { createContext, useState, useEffect, useContext, ReactNode } from "react";
import { readDir } from "@tauri-apps/api/fs";

interface ProjectsContextType {
  projectsPath: string | null;
  setProjectsPath: React.Dispatch<React.SetStateAction<string | null>>;
  projects: string[];
  error: string | null;
  platform: string | null;
  onboard: boolean;
  setOnboard: React.Dispatch<React.SetStateAction<boolean>>;
}

const ProjectsContext = createContext<ProjectsContextType | undefined>(undefined);

export const ProjectsProvider = ({ children }: { children: ReactNode }) => {
  const [onboard, setOnboard] = useState<boolean>(() => {
    if (typeof window !== 'undefined') {
      const storedOnboard = localStorage.getItem('onboard');
      return storedOnboard ? JSON.parse(storedOnboard) : false;
    }
    return false;
  });
  const [projectsPath, setProjectsPath] = useState<string | null>(() => {
    if (typeof window !== 'undefined') {
      const storedPath = localStorage.getItem("projectsPath");
      return storedPath || null;
    }
    return null;
  });
  
  const [projects, setProjects] = useState<string[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [platform, setPlatform] = useState<string | null>(null);

  useEffect(() => {
    const fetchPlatform = async () => {
      if (typeof window !== 'undefined') {
        const oss = await import("@tauri-apps/api/os");
        const osPlatform = await oss.platform();
        console.log("Platform in useProject", osPlatform)
        setPlatform(osPlatform);
        console.log("Platform in useProject after setPlatform", platform)
      }
    };

    fetchPlatform();
  }, []);

  useEffect(() => {
    const fetchProjects = async () => {
      if (projectsPath) {
        try {
          const entries = await readDir(projectsPath);
          setProjects(
            entries
              .map((entry) => entry.name)
              .filter((name): name is string => name !== undefined)
          );
          setError(null); // Reset error on successful fetch
        } catch (err) {
          console.error("Error reading projects directory:", err);
          setError("Failed to load projects. Please check your projects directory.");
        }
      } else {
        setError("Projects path not set. Please set a valid projects directory.");
      }
    };

    fetchProjects();
  }, [projectsPath]); // Runs whenever projectsPath changes

  if (platform === null) {
    return null; // or a loading spinner, or some other placeholder
  }
  return (
    <ProjectsContext.Provider value={{ projectsPath, setProjectsPath, projects, error, platform, onboard, setOnboard }}>
      {children}
    </ProjectsContext.Provider>
  );
};

export const useProjects = () => {
  const context = useContext(ProjectsContext);
  if (context === undefined) {
    throw new Error('useProjects must be used within a ProjectsProvider');
  }
  return context;
};

export default ProjectsContext;
