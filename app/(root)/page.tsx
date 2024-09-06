"use client";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { useProjects } from "@/lib/useProject";
import { useRouter } from "next/navigation";
import { open } from "@tauri-apps/api/dialog";
import { Input } from '@/components/ui/input';
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { invoke } from '@tauri-apps/api/tauri';



const DashboardPage = () => {
  const { projects, error,platform } = useProjects();
  const [loading, setLoading] = useState(true);
  const [projectsPath, setProjectsPath] = useState<string | null>(null);
  const [existingFolder, setExistingFolder] = useState<string>('');
  const [newFolderName, setNewFolderName] = useState<string>('');
  const [location, setLocation] = useState<string>('');
  const [showNewFolderInput, setShowNewFolderInput] = useState<boolean>(false);
  const [errorState, setErrorState] = useState<string | null>(null);
  const [newProjectName, setNewProjectName] = useState<string>('');
  const router = useRouter();

  useEffect(() => {
    const storedProjectsPath = localStorage.getItem('projectsPath');
    if (storedProjectsPath) {
      setProjectsPath(storedProjectsPath);
      
    }
    console.log('projectsPath', projectsPath);
    setLoading(false);
  }, []);

  const handleExistingFolderChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setExistingFolder(event.target.value);
  };

  const handleNewFolderNameChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setNewFolderName(event.target.value);
  };

  const handleNewProjectNameChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setNewProjectName(event.target.value);
  };

  const handleBrowseExisting = async () => {
    try {
      const selectedLocation = await open({
        directory: true,
        multiple: false,
        defaultPath: location || projectsPath || '',
      });
      if (selectedLocation) {
        let formattedLocation = selectedLocation as string;
        
        
        if (platform === 'win32') {
          formattedLocation = formattedLocation.replace(/\//g, '\\\\');
          console.log('formattedLocation', formattedLocation);
        } else {
          console.log('formattedLocation not on windows', formattedLocation);
          formattedLocation = formattedLocation.replace(/\\/g, '/');
        }
        setExistingFolder(formattedLocation);
      }
    } catch (err) {
      console.error("Error selecting location:", err);
      setErrorState("Failed to select location. Please try again.");
    }
  };

  const handleBrowseNew = async () => {
    try {
      const selectedLocation = await open({
        directory: true,
        multiple: false,
        defaultPath: location || projectsPath || '',
      });
      if (selectedLocation) {
        let formattedLocation = selectedLocation as string;
        
      
        if (platform === 'win32') {
          formattedLocation = formattedLocation.replace(/\//g, '\\\\');
        } else {
          formattedLocation = formattedLocation.replace(/\\/g, '/');
        }
        setNewFolderName(formattedLocation);
        setShowNewFolderInput(true);
      }
    } catch (err) {
      console.error("Error selecting location:", err);
      setErrorState("Failed to select location. Please try again.");
    }
  };

  const handleSave = async () => {
    let projectPath;
    if (showNewFolderInput && newProjectName) {
      if (platform === 'win32') {
        projectPath = `${newFolderName}\\${newProjectName}`;
      } else {
        projectPath = `${newFolderName}/${newProjectName}`;
      }
    } else {
      projectPath = existingFolder;
    }

    if (projectPath) {
      console.log(projectPath);
      localStorage.setItem('projectsPath', projectPath);
      
      if (showNewFolderInput && newProjectName) {
        try {
          const createdPath = await invoke('create_local_projects_folder', {
            folderName: newProjectName,
            path: newFolderName
          });
          console.log(`Created path: ${createdPath}`);
        } catch (error) {
          console.error(`Failed to create folder: ${error}`);
        }
      }
      
      // Fallback to window.location.reload() if router.refresh() doesn't work
      window.location.reload();
    }
  };

  const handleTabChange = () => {
    setExistingFolder('');
    setNewFolderName('');
    setShowNewFolderInput(false);
    setNewProjectName('');
  };

  if (loading) {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-900 text-purple-200">
        <h1 className="text-3xl font-bold mb-6">Loading...</h1>
      </div>
    );
  }

  return !projectsPath ? (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-purple-200 p-6">
    <h1 className="text-4xl font-bold mb-8 text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-purple-600">Welcome to Local</h1>
    <Tabs defaultValue="existing" className="w-full max-w-md bg" onValueChange={handleTabChange}>
      <TabsList className="grid w-full  grid-cols-2 mb-8 bg-gray-800">
        <TabsTrigger value="existing" className="px-3 py-2 rounded-l-lg  bg-gray-800 text-white data-[state=active]:bg-purple-700 data-[state=active]:text-white transition-colors">
          Choose Existing
        </TabsTrigger>
        <TabsTrigger value="new" className="px-3 py-2 rounded-r-lg bg-gray-800 text-white data-[state=active]:bg-purple-700 data-[state=active]:text-whitetransition-colors">
          Create New
        </TabsTrigger>
      </TabsList>
      <TabsContent value="existing" className="mt-4">
        <h2 className="text-2xl font-semibold mb-4">Choose Existing Work Folder</h2>
        <div className="flex gap-2 mb-4">
          <Input
            type="text"
            value={existingFolder}
            onChange={handleExistingFolderChange}
            placeholder="Existing folder path"
            className="flex-grow bg-gray-800 text-white border-purple-500 focus:border-purple-400"
          />
          <Button onClick={handleBrowseExisting} className="bg-purple-600 hover:bg-purple-500 text-white">
            Browse
          </Button>
        </div>
      </TabsContent>
      <TabsContent value="new" className="mt-4">
        <h2 className="text-2xl font-semibold mb-4">Create New Work Folder</h2>
        <div className="flex gap-2 mb-4">
          <Input
            type="text"
            value={newFolderName}
            onChange={handleNewFolderNameChange}
            placeholder="New folder name"
            className="flex-grow bg-gray-800 text-white border-purple-500 focus:border-purple-400"
          />
          <Button onClick={handleBrowseNew} className="bg-purple-600 hover:bg-purple-500 text-white">
            Browse
          </Button>
        </div>
        {showNewFolderInput && (
          <div className="flex gap-2 mb-4">
            <Input
              type="text"
              value={newProjectName}
              onChange={handleNewProjectNameChange}
              placeholder="New project name"
              className="flex-grow bg-gray-800 text-white border-purple-500 focus:border-purple-400"
            />
          </div>
        )}
      </TabsContent>
    </Tabs>
    <Button 
      onClick={handleSave} 
      className="mt-8 px-8 py-3 bg-gradient-to-r from-purple-600 to-purple-800 text-white rounded-lg shadow-lg hover:from-purple-700 hover:to-purple-900 transition-all transform hover:scale-105"
      disabled={(!existingFolder && !showNewFolderInput) || (showNewFolderInput && !newProjectName)}
    >
      Save
    </Button>
  </div>
  ) : (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-900 text-purple-200">
      {error ? (
        <>
          <h1 className="text-3xl font-bold mb-6">Error</h1>
          <p className="text-red-400 mb-6">{error}</p>
          <Button
            onClick={() => router.replace("/create")}
            className="bg-gradient-to-r from-purple-500 to-purple-700 text-white px-6 py-3 rounded-lg shadow-lg hover:from-purple-600 hover:to-purple-800 transform hover:scale-105 transition-all"
          >
            Create New Site
          </Button>
        </>
      ) : projects.length === 0 ? (
        <>
          <h1 className="text-3xl font-bold mb-6">Create New Site</h1>
          <Button
            onClick={() => router.replace("/create")}
            className="bg-gradient-to-r from-purple-500 to-purple-700 text-white px-6 py-3 rounded-lg shadow-lg hover:from-purple-600 hover:to-purple-800 transform hover:scale-105 transition-all"
          >
            Create Site
          </Button>
        </>
      ) : (
        <>
          <h1 className="text-3xl font-bold mb-6">Pick a Project from Sidebar</h1>
          
        </>
      )}
    </div>
  );
};

export default DashboardPage;