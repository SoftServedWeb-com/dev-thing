"use client"
import React, { createContext, useState, useEffect, useContext, ReactNode } from "react";
import { readDir } from "@tauri-apps/api/fs";

interface ProjectsContextType {
  projectsPath: string;
  setProjectsPath: React.Dispatch<React.SetStateAction<string>>;
  projects: string[];
  error: string | null;
}

const ProjectsContext = createContext<ProjectsContextType | undefined>(undefined);

export const ProjectsProvider = ({ children }: { children: ReactNode }) => {
  const [projectsPath, setProjectsPath] = useState(() => {
    if (typeof window !== 'undefined') {
      const storedPath = localStorage.getItem("projectsPath") || "/";
      return storedPath;
    }
    return "";
  });
  
  const [projects, setProjects] = useState<string[]>([]);
  const [error, setError] = useState<string | null>(null);

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

  return (
    <ProjectsContext.Provider value={{ projectsPath, setProjectsPath, projects, error }}>
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
