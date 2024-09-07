// components/ComboboxDemo.tsx
"use client";

import * as React from "react";
import { Check, ChevronsUpDown } from "lucide-react";

import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";

const frameworks = [
  { value: "next.js", label: "Next.js" },
  { value: "vue", label: "Vue" },
  { value: "nuxt", label: "Nuxt" },
];

interface ComboboxDemoProps {
  onSelect: (value: string) => void;
  disabled?: boolean;
}

export function ComboboxDemo({ onSelect, disabled }: ComboboxDemoProps) {
  const [open, setOpen] = React.useState(false);
  const [value, setValue] = React.useState("");

  return (
    <Popover open={open} onOpenChange={setOpen} >
      <PopoverTrigger asChild >
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          disabled = {disabled}
          className="w-[250px] justify-between bg-gray-900 text-white border-gray-700 hover:bg-purple-500"
        >
          {value
            ? frameworks.find((framework) => framework.value === value)?.label
            : "Select framework..."}
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[250px] p-0 ">
        <Command className="bg-gray-900 ">
          <CommandInput className="bg-gray-900 text-white" placeholder="Search framework..." />
          <CommandList >
            <CommandEmpty className="text-white">No framework found.</CommandEmpty>
            <CommandGroup className="text-white">
              {frameworks.map((framework) => (
                <CommandItem
                  key={framework.value}
                  
                  value={framework.value}
                  onSelect={(currentValue) => {
                    setValue(currentValue);
                    onSelect(currentValue);
                    setOpen(false);
                
                  }}
                >
                  <Check
                    className={cn(
                      "mr-2 h-4 w-4",
                      value === framework.value ? "opacity-100" : "opacity-0"
                    )}
                  />
                  {framework.label}
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
