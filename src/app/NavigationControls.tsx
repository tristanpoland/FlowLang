import React from 'react';
import { Button } from './components/ui/Button';
import { ZoomIn, ZoomOut, Move } from 'lucide-react';

interface NavigationControlsProps {
  onZoomIn: () => void;
  onZoomOut: () => void;
  onResetZoom: () => void;
  onResetPan: () => void;
  zoom: number;
}

const NavigationControls = ({
  onZoomIn,
  onZoomOut,
  onResetZoom,
  onResetPan,
  zoom
}: NavigationControlsProps) => (
  <div className="absolute bottom-4 right-4 flex flex-col space-y-2 bg-neutral-800 p-2 rounded-lg shadow-lg">
    <Button 
      variant="outline" 
      size="icon" 
      onClick={onZoomIn} 
      className="border-neutral-600 hover:bg-neutral-700"
    >
      <ZoomIn className="h-4 w-4" />
    </Button>
    
    <Button 
      variant="outline" 
      size="icon" 
      onClick={onZoomOut} 
      className="border-neutral-600 hover:bg-neutral-700"
    >
      <ZoomOut className="h-4 w-4" />
    </Button>
    
    <Button 
      variant="outline" 
      size="icon" 
      onClick={() => { 
        onResetZoom();
        onResetPan();
      }} 
      className="border-neutral-600 hover:bg-neutral-700"
    >
      <Move className="h-4 w-4" />
    </Button>
    
    <div className="text-center text-sm text-neutral-400">
      {Math.round(zoom * 100)}%
    </div>
  </div>
);

export default NavigationControls;