import React from "react";

export interface LogoProps {
  size?: "sm" | "md" | "lg" | "xl";
  variant?: "full" | "icon" | "short";
  className?: string;
}

/**
 * Logo - CDSP 79 Official Logo Component
 * Based on the official logo: "Le PALET en DEUX-SÈVRES"
 * Colors: Green (#00A651), Orange (#FF8C00), Gray (#666666)
 */
export function Logo({ size = "md", variant = "full", className = "" }: LogoProps) {
  const getSizeClasses = () => {
    switch (size) {
      case "sm":
        return "w-8 h-8 text-sm";
      case "md":
        return "w-12 h-12 text-base";
      case "lg":
        return "w-16 h-16 text-lg";
      case "xl":
        return "w-20 h-20 text-xl";
      default:
        return "w-12 h-12 text-base";
    }
  };

  const getVariant = () => {
    switch (variant) {
      case "icon":
        return <LogoIcon size={size} className={className} />;
      case "short":
        return <LogoShort size={size} className={className} />;
      default:
        return <LogoFull size={size} className={className} />;
    }
  };

  return <div className={`logo ${getSizeClasses()} ${className}`}>{getVariant()}</div>;
}

/**
 * Full Logo - "Le PALET en DEUX-SÈVRES" with all elements
 */
function LogoFull({ size, className }: { size: string; className?: string }) {
  const scale = size === "sm" ? 0.6 : size === "md" ? 0.8 : size === "xl" ? 1.4 : 1;

  return (
    <div 
      className={`flex items-center justify-center ${className}`}
      style={{ transform: `scale(${scale})` }}
    >
      <svg 
        viewBox="0 0 400 120" 
        className="w-full h-auto"
        xmlns="http://www.w3.org/2000/svg"
      >
        {/* Tree - Green */}
        <g transform="translate(320, 20)">
          <ellipse cx="40" cy="20" rx="35" ry="25" fill="#00A651" />
          <ellipse cx="35" cy="15" rx="30" ry="20" fill="#008a42" />
          <ellipse cx="45" cy="25" rx="28" ry="18" fill="#006a32" />
          <circle cx="55" cy="5" r="8" fill="#00A651" />
          <circle cx="65" cy="8" r="6" fill="#008a42" />
          <circle cx="70" cy="12" r="5" fill="#006a32" />
          <circle cx="40" cy="3" r="7" fill="#00A651" />
        </g>

        {/* Silhouette - Orange */}
        <g transform="translate(250, 40)">
          <path 
            d="M0,20 C5,-5 15,-10 25,0 C35,10 25,25 15,35 C5,45 0,40 0,20 Z" 
            fill="#FF8C00"
          />
          <circle cx="10" cy="8" r="3" fill="#FF8C00" />
        </g>

        {/* "7" - Orange */}
        <g transform="translate(200, 40)">
          <circle cx="25" cy="25" r="28" fill="#FF8C00" />
          <circle cx="25" cy="25" r="22" fill="#e07a00" />
          <text 
            x="25" y="32" 
            fontFamily="Inter, sans-serif" 
            fontSize="24" 
            fontWeight="900" 
            fill="white" 
            textAnchor="middle"
            dominantBaseline="middle"
          >
            7
          </text>
        </g>

        {/* Star - Gray */}
        <g transform="translate(180, 30)">
          <polygon 
            points="25,0 30,18 45,18 33,28 38,45 25,35 12,45 17,28 5,18 20,18" 
            fill="#666666"
          />
        </g>

        {/* "9" - Green */}
        <g transform="translate(140, 40)">
          <circle cx="25" cy="25" r="28" fill="#00A651" />
          <circle cx="25" cy="25" r="22" fill="#008a42" />
          <text 
            x="25" y="32" 
            fontFamily="Inter, sans-serif" 
            fontSize="24" 
            fontWeight="900" 
            fill="white" 
            textAnchor="middle"
            dominantBaseline="middle"
          >
            9
          </text>
        </g>

        {/* Text "Le" - Green */}
        <text 
          x="20" y="55" 
          fontFamily="Inter, sans-serif" 
          fontSize="28" 
          fontWeight="700" 
          fill="#00A651" 
          textAnchor="start"
        >
          Le
        </text>

        {/* Text "PALET en" - Gray */}
        <text 
          x="50" y="55" 
          fontFamily="Inter, sans-serif" 
          fontSize="28" 
          fontWeight="700" 
          fill="#666666" 
          textAnchor="start"
        >
          PALET en
        </text>

        {/* Text "DEUX-SÈVRES" - Green */}
        <text 
          x="130" y="90" 
          fontFamily="Inter, sans-serif" 
          fontSize="28" 
          fontWeight="700" 
          fill="#00A651" 
          textAnchor="start"
        >
          DEUX-SÈVRES
        </text>

        {/* Background shape */}
        <path 
          d="M10,80 L380,80 L350,110 L50,110 Z" 
          fill="none" 
          stroke="#666666" 
          strokeWidth="3"
        />
      </svg>
    </div>
  );
}

/**
 * Short Logo - "CDSP 79" with colors
 */
function LogoShort({ size, className }: { size: string; className?: string }) {
  const sizeClass = size === "sm" ? "text-2xl" : size === "md" ? "text-3xl" : size === "lg" ? "text-4xl" : "text-5xl";

  return (
    <div className={`flex items-center gap-2 ${className}`}>
      <div className={`w-10 h-10 ${size === "sm" ? "w-8 h-8" : size === "lg" ? "w-14 h-14" : size === "xl" ? "w-16 h-16" : "w-12 h-12"} rounded-full border-2 border-[var(--color-brand-green)] flex items-center justify-center bg-black`}>
        <span className={`font-black ${size === "sm" ? "text-sm" : size === "md" ? "text-base" : size === "lg" ? "text-lg" : "text-xl"}`} style={{color: "#00A651"}}>
          79
        </span>
      </div>
      <div className={`flex flex-col ${size === "sm" ? "hidden" : ""}`}>
        <span className={`font-bold ${sizeClass} tracking-tight text-white`}>
          CDSP
        </span>
        <span className={`text-xs text-[var(--text-tertiary)] tracking-wider uppercase ${size === "md" ? "hidden" : ""}`}>
          Deux-Sèvres
        </span>
      </div>
    </div>
  );
}

/**
 * Icon Logo - Just the "79" in a circle
 */
function LogoIcon({ size, className }: { size: string; className?: string }) {
  const iconSize = size === "sm" ? 32 : size === "md" ? 48 : size === "lg" ? 64 : 80;

  return (
    <div 
      className={`w-${iconSize/4}r h-${iconSize/4}r rounded-full border-2 border-[var(--color-brand-green)] flex items-center justify-center bg-black ${className}`}
      style={{ width: `${iconSize}px`, height: `${iconSize}px` }}
    >
      <span 
        className="font-black text-white" 
        style={{ fontSize: `${iconSize * 0.4}px` }}
      >
        79
      </span>
    </div>
  );
}

export default Logo;
