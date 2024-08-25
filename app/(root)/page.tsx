"use client";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { useProjects } from "@/lib/useProject";
import { useRouter } from "next/navigation";

const DashboardPage = () => {
  const { projects, error } = useProjects();
  const [loading, setLoading] = useState(true);
  const router = useRouter();

  useEffect(() => {
    if (projects.length >= 0 || error) {
      setLoading(false);
    }
  }, [projects, error]);

  const handleCreateClick = () => {
    router.push("/create");
  };

  if (loading) {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-900 text-purple-200">
        <h1 className="text-3xl font-bold mb-6">Loading...</h1>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-900 text-purple-200">
      {error ? (
        <>
          <h1 className="text-3xl font-bold mb-6">Error</h1>
          <p className="text-red-400 mb-6">{error}</p>
          <Button
            onClick={handleCreateClick}
            className="bg-gradient-to-r from-purple-500 to-purple-700 text-white px-6 py-3 rounded-lg shadow-lg hover:from-purple-600 hover:to-purple-800 transform hover:scale-105 transition-all"
          >
            Create New Site
          </Button>
        </>
      ) : projects.length === 0 ? (
        <>
          <h1 className="text-3xl font-bold mb-6">Create New Site</h1>
          <Button
            onClick={handleCreateClick}
            className="bg-gradient-to-r from-purple-500 to-purple-700 text-white px-6 py-3 rounded-lg shadow-lg hover:from-purple-600 hover:to-purple-800 transform hover:scale-105 transition-all"
          >
            Create Site
          </Button>
        </>
      ) : (
        <>
          <h1 className="text-3xl font-bold mb-6">Pick a Project</h1>
          <ul className="mb-6">
            {projects.map((project, index) => (
              <li key={index} className="mb-2 text-purple-300">
                <Button
                  onClick={() => window.location.href = `/project/?page=${encodeURIComponent(project)}`}
                  variant="ghost"
                  className="hover:text-purple-700 transition-colors"
                >
                  {project}
                </Button>
              </li>
            ))}
          </ul>
        </>
      )}
    </div>
  );
};

export default DashboardPage;
