import React from 'react';
import { Button } from './components/ui/Button';
import { Input } from './components/ui/Input';
import { Plus, FileJson } from 'lucide-react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuSeparator
} from "./components/ui/DropdownMenu";
import { RUST_TYPES } from './constants';

interface TitleBarProps {
  searchTerm: string;
  onSearchChange: (value: string) => void;
  onAddNode: (typeKey: string, typeInfo: any) => void;
  onExport: () => void;
}

const TitleBar = ({ searchTerm, onSearchChange, onAddNode, onExport }: TitleBarProps) => {
  // Filter infrastructure types based on search
  const filteredTypes = Object.entries(RUST_TYPES).reduce((acc, [category, types]) => {
    const filtered = Object.entries(types).filter(([key, value]) =>
      value.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      value.description.toLowerCase().includes(searchTerm.toLowerCase()) ||
      key.toLowerCase().includes(searchTerm.toLowerCase())
    );
    if (filtered.length > 0) {
      acc[category] = Object.fromEntries(filtered);
    }
    return acc;
  }, {});

  const handleNodeAdd = (typeKey: string, typeInfo: any) => {
    console.log('Adding node:', { typeKey, typeInfo });
    onAddNode(typeKey, typeInfo);
    onSearchChange('');
  };

  return (
    <div className="h-16 border-b border-neutral-800 flex items-center justify-between px-4 bg-neutral-900">
      <h1 className="text-xl font-semibold">Infrastructure Designer</h1>
      <div className="flex space-x-4">
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button className="flex items-center bg-neutral-800 hover:bg-neutral-700">
              <Plus className="w-4 h-4 mr-2" />
              Add Resource
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="w-64 max-h-96 overflow-y-auto bg-neutral-800 border-neutral-700">
            <div className="p-2 sticky top-0 bg-neutral-800 border-b border-neutral-700">
              <Input
                placeholder="Search resources..."
                value={searchTerm}
                onChange={(e) => onSearchChange(e.target.value)}
                className="bg-neutral-900 border-neutral-700"
              />
            </div>
            {Object.entries(filteredTypes).map(([category, types]) => (
              <div key={category}>
                <div className="px-2 py-1 text-sm font-semibold bg-neutral-800 sticky top-14">
                  {category.toUpperCase()}
                </div>
                {Object.entries(types).map(([typeKey, typeInfo]) => (
                  <DropdownMenuItem
                    key={typeKey}
                    onSelect={() => handleNodeAdd(typeKey, typeInfo)}
                    className="flex flex-col items-start hover:bg-neutral-700"
                  >
                    <span className="font-medium">{typeInfo.name}</span>
                    <span className="text-xs text-neutral-400">{typeInfo.description}</span>
                  </DropdownMenuItem>
                ))}
                <DropdownMenuSeparator className="bg-neutral-700" />
              </div>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>

        <Button
          onClick={onExport}
          className="flex items-center bg-neutral-800 hover:bg-neutral-700"
          variant="outline"
        >
          <FileJson className="w-4 h-4 mr-2" />
          Export JSON
        </Button>
      </div>
    </div>
  );
};

export default TitleBar;