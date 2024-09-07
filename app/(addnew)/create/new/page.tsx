"use client";
import { use, useEffect, useRef, useState } from "react";
import { Button } from "@/components/ui/button";
import { ComboboxDemo } from "@/components/ui/combobox";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';
import { useRouter } from "next/navigation";

const NewSitePage = () => {
  const router = useRouter();
  const [projectsPath, setProjectsPath] = useState('');
  const [location, setLocation] = useState(projectsPath);
  
  const [projectName, setProjectName] = useState("");
  const [runtime, setRuntime] = useState("");
  const [framework, setFramework] = useState("");
  
  const [creationStatus, setCreationStatus] = useState("");
  const [error, setError] = useState<string | null>(null);

  const projectNameRef = useRef(projectName);

  useEffect(() => {
    projectNameRef.current = projectName;
  }, [projectName]);

  useEffect(() => {
    setLocation(localStorage.getItem('projectsPath') || '');
    const unlisten = listen('creation_status', (event: any) => {
      setCreationStatus(event.payload as string);
    });
  
    return () => {
      unlisten.then(f => f());
    };
  }, [projectName]);

  useEffect(() => {
    if (creationStatus === "Project created successfully!") {
      const path = `/project/?page=${projectNameRef.current}`;
      router.push(path );

    }
  }, [creationStatus, router]);
  
  const handleCreateClick = async () => {
    if (projectName && runtime && framework && location) {
      setCreationStatus("Starting project creation...");
      setError(null);
      try {
        console.log("Creating project...", projectName, runtime, framework, location);
        await invoke("start_project_creation", {
          projectName,
          runtime,
          framework,
          location,
        });
      } catch (err: any) {
        console.error("Error starting project creation:", err);
        setError(`Error: ${err.message || "An error occurred while starting project creation"}`);
        setCreationStatus("");
      }
    } else {
      setError("Please fill in all fields before creating the project.");
    }
  };

  const handleGoBackClick = () => {
    router.back();
  };

  const handlePickLocation = async () => {
    try {
      const selectedLocation = await open({
        directory: true,
        multiple: false,
        defaultPath: location || projectsPath,
      });
      if (selectedLocation) {
        setLocation(selectedLocation as string);
      }
    } catch (err) {
      console.error("Error selecting location:", err);
      setError("Failed to select location. Please try again.");
    }
  };

  return (
    <div className="w-screen max-w-lg mx-auto p-6  bg-gray-900 text-white rounded-lg shadow-lg">
      <div className="flex justify-between items-center mb-6">
        <Button
          onClick={handleGoBackClick}
          disabled={creationStatus === "Starting project creation..."}
          className="absolute top-4 right-4 bg-gradient-to-r from-purple-500 to-purple-700 text-white px-4 py-2 shadow-lg hover:from-purple-600 hover:to-purple-800 transform hover:scale-105 transition-all"
        >
          Go Back
        </Button>
        <h1 className="text-2xl font-semibold">Create New Site</h1>
      </div>
      <div className="mb-4">
        <Label className="block text-gray-300 mb-2" htmlFor="projectName">
          Project Name
        </Label>
        <Input
          id="projectName"
          type="text"
          value={projectName}
          disabled={creationStatus === "Starting project creation..."}
          onChange={(e) => setProjectName(e.target.value)}
          placeholder="Enter project name"
          className="w-full p-2 rounded-lg bg-gray-800 border border-gray-700 text-white placeholder-gray-400"
        />
      </div>
      <div className="mb-4">
        <Label className="block text-gray-300 mb-2" htmlFor="runtime">
          Choose Runtime
        </Label>
        <div className="flex gap-4">
          {[ "pnpm", "npm", "yarn"].map((r) => (
            <Button
              key={r}
              onClick={() => setRuntime(r)}
              disabled={creationStatus === "Starting project creation..."}
              className={`w-full py-2 rounded-lg ${
                runtime === r ? "bg-purple-700" : "bg-gray-800"
              } hover:bg-purple-600 transition-all`}
            >
              {r}
            </Button>
          ))}
        </div>
      </div>
      <div className="mb-4">
        <Label className="block text-gray-300 mb-2" htmlFor="location">
          Your Project Location
        </Label>
        <div className="flex items-center">
          <Input
            id="location"
            type="text"
            value={location}
            
            readOnly
            placeholder="Choose location"
            disabled={true}
            className="w-full p-2 rounded-lg bg-gray-800 border border-gray-700 text-white placeholder-gray-400"
          />
          <Button
            onClick={handlePickLocation}
            disabled={true}
            className="ml-4 bg-purple-700 hover:bg-purple-600 text-white px-4 py-2 rounded-lg"
          >
            Browse
          </Button>
        </div>
      </div>
      <div className="mb-6">
        <Label className="block text-gray-300 mb-2">Choose Framework</Label>
        <ComboboxDemo 
          disabled={creationStatus === "Starting project creation..."} 
          onSelect={(value) => setFramework(value)} 
        />
      </div>
      {error && (
        <p className="text-sm mb-4 text-red-500">{error}</p>
      )}
      {creationStatus && (
        <p className={`text-sm mb-4 ${creationStatus.includes("Error") ? "text-red-500" : "text-green-500"}`}>
          {creationStatus}
        </p>
      )}
      <div className="flex flex-col items-center">
        <Button
          onClick={handleCreateClick}
          disabled={!projectName || !runtime || !framework || creationStatus === "Starting project creation..."}
          className="bg-gradient-to-r from-blue-500 to-purple-600 text-white px-6 py-3 rounded-lg shadow-lg hover:from-blue-600 hover:to-purple-700 transform hover:scale-105 transition-all disabled:opacity-50 w-full"
        >
          {creationStatus === "Starting project creation..." ? "Creating..." : "Create"}
        </Button>
      </div>
    </div>
  );
};

export default NewSitePage;