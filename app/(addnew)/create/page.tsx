"use client";
import { useState } from "react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { AiOutlineClose } from "react-icons/ai";

const CreatePage = () => {
  const [selectedOption, setSelectedOption] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const router = useRouter();

  const handleOptionClick = (option: string) => {
    setSelectedOption(option);
    setError(null);
  };

  const handleContinueClick = () => {
    if (selectedOption) {
      router.push(`/create/${selectedOption}`);
    } else {
      setError("Please select an option before continuing.");
    }
  };

  const handleCloseClick = () => {
 
    router.back();
  };

  return (
    <div className="w-full max-w-lg mx-auto p-6 bg-gray-900 text-white rounded-lg shadow-lg relative">
      <Button
        onClick={handleCloseClick}
        className="absolute top-4 right-4 text-gray-400 hover:text-white transition-colors"
      >
        <AiOutlineClose size={24} />
      </Button>
      <h1 className="text-2xl font-semibold mb-8">Choose an Option</h1>
      <div className="space-y-4">
        <Button
          onClick={() => handleOptionClick("new")}
          className={`w-full py-4 rounded-lg ${
            selectedOption === "new" ? "bg-purple-600" : "bg-gray-800"
          } hover:bg-purple-600 transition-all`}
        >
          Create New Site
        </Button>
        <Button
          onClick={() => handleOptionClick("blueprint")}
          disabled={true}
          className={`w-full py-4 rounded-lg ${
            selectedOption === "blueprint" ? "bg-purple-600" : "bg-gray-800"
          } hover:bg-purple-600 transition-all`}
        >
          Create from Blueprint
        </Button>
      </div>
      {error && <p className="text-red-500 mt-4">{error}</p>}
      <div className="mt-8 text-left">
        <Button
          onClick={handleContinueClick}
          disabled={!selectedOption}
          className="bg-gradient-to-r from-blue-500 to-purple-600 text-white px-6 py-3 rounded-lg shadow-lg hover:from-blue-600 hover:to-purple-700 transform hover:scale-105 transition-all disabled:opacity-50"
        >
          Continue
        </Button>
      </div>
    </div>
  );
};

export default CreatePage;