"use client"
import React, { use, useContext, useEffect, useState } from 'react';
import { Folder, User, Plus, CircleChevronRight } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from './ui/tooltip';
import { Button } from './ui/button';
import Link from 'next/link';
import { readDir } from '@tauri-apps/api/fs';
import { ScrollArea } from './ui/scroll-area';
import { useProjects } from '@/lib/useProject';
import { useRouter } from 'next/navigation';


const Sidebar = () => {
  const { projects, error } = useProjects();

  const [isProjectsOpen, setIsProjectsOpen] = useState(false);

  

  
  const toggleLocalSites = () => {
    setIsProjectsOpen(prevState => !prevState);
  };

  const router = useRouter();

  const handleProjectClick = (project: string) => {
    console.log("Project clicked:", project);
    router.push(`/project/?page=${encodeURIComponent(project)}`);
    
  }

  return (
    <TooltipProvider>
      <div className="flex h-screen bg-slate-800 text-white">
        <div className="w-16 bg-slate-800 flex flex-col items-center py-4">
          <Tooltip>
            <TooltipTrigger asChild>
              <Button className="mb-8 p-2 rounded-full hover:bg-purple-700" aria-label="User Profile">
                <User size={28} />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>View and edit your profile</p>
            </TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                className={`mb-4 p-2 rounded-lg ${isProjectsOpen ? 'bg-purple-700' : ''} hover:bg-purple-600`}
                onClick={toggleLocalSites}
                aria-label="Toggle Local Sites"
              >
                <Folder size={24} />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>Toggle local sites</p>
            </TooltipContent>
          </Tooltip>

          <div className="mt-auto w-full px-4">
            <Tooltip>
              <TooltipTrigger asChild>
                <Link href="/create">
                  <Button
                    className="mb-1 p-2 rounded-sm bg-purple-600 hover:bg-purple-700 transition-all duration-150"
                    aria-label="Create New Project"
                  >
                    <Plus className="h-5 w-5" />
                  </Button>
                </Link>
              </TooltipTrigger>
              <TooltipContent>
                <p>Create a new project</p>
              </TooltipContent>
            </Tooltip>
          </div>
        </div>

        {isProjectsOpen && (
          <div className="w-64 bg-zinc-900 p-4">
            <h2 className="text-xl font-semibold mb-4 text-purple-300">Local sites</h2>
            <ScrollArea className="h-[calc(100vh-8rem)] w-full mt-2">
              {error ? (
                <p className="text-red-400">{error}</p>
              ) : projects.length > 0 ? (
                <div id="projects-list" className="space-y-1">
                  {projects.map((project, index) => (
                   
                      <Button
                        variant="ghost"
                        size="sm"
                        key={index}
                        onClick={() => handleProjectClick(project)}
                        className="w-full justify-start pl-4 hover:bg-zinc-400 transition-all duration-150"
                      >
                        <CircleChevronRight className="mr-2 h-5 w-5" />
                        {project}
                      </Button>
                  
                  ))}
                </div>
              ) : (
                <p className="text-gray-400">No projects found.</p>
              )}
            </ScrollArea>
          </div>
        )}
      </div>
    </TooltipProvider>
  );
};

export default Sidebar;