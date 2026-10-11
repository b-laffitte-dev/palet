import { useState, useEffect } from "react";

/**
 * Device types based on screen width
 */
export type DeviceType = "mobile" | "tablet" | "desktop" | "largeDesktop";

/**
 * Breakpoints for responsive design
 */
export const BREAKPOINTS = {
  MOBILE: 639,
  TABLET: 767,
  DESKTOP: 1023,
  LARGE_DESKTOP: 1279,
} as const;

/**
 * Hook to detect current device type and screen size
 * Returns the device type and whether the screen is at least a certain size
 */
export function useResponsive() {
  const [device, setDevice] = useState<DeviceType>(
    getDeviceType(window.innerWidth)
  );
  const [width, setWidth] = useState(window.innerWidth);
  const [height, setHeight] = useState(window.innerHeight);

  // Determine device type based on width
  function getDeviceType(width: number): DeviceType {
    if (width <= BREAKPOINTS.MOBILE) return "mobile";
    if (width <= BREAKPOINTS.TABLET) return "tablet";
    if (width <= BREAKPOINTS.DESKTOP) return "desktop";
    return "largeDesktop";
  }

  // Helper functions to check breakpoints
  const isMobile = () => device === "mobile";
  const isTablet = () => device === "tablet";
  const isDesktop = () => device === "desktop" || device === "largeDesktop";
  const isLargeDesktop = () => device === "largeDesktop";
  const isMobileOrTablet = () => isMobile() || isTablet();

  // Check if screen is at least a certain breakpoint
  const isAtLeast = {
    sm: (minWidth: number = BREAKPOINTS.MOBILE + 1) => width >= minWidth,
    md: (minWidth: number = BREAKPOINTS.TABLET + 1) => width >= minWidth,
    lg: (minWidth: number = BREAKPOINTS.DESKTOP + 1) => width >= minWidth,
    xl: (minWidth: number = BREAKPOINTS.LARGE_DESKTOP + 1) => width >= minWidth,
  };

  // Check if screen is less than a certain breakpoint
  const isLessThan = {
    sm: (maxWidth: number = BREAKPOINTS.MOBILE) => width <= maxWidth,
    md: (maxWidth: number = BREAKPOINTS.TABLET) => width <= maxWidth,
    lg: (maxWidth: number = BREAKPOINTS.DESKTOP) => width <= maxWidth,
    xl: (maxWidth: number = BREAKPOINTS.LARGE_DESKTOP) => width <= maxWidth,
  };

  // Orientation
  const [orientation, setOrientation] = useState<"portrait" | "landscape">(
    window.innerHeight > window.innerWidth ? "portrait" : "landscape"
  );

  // Handle resize
  useEffect(() => {
    const handleResize = () => {
      const newWidth = window.innerWidth;
      const newHeight = window.innerHeight;
      
      setWidth(newWidth);
      setHeight(newHeight);
      setDevice(getDeviceType(newWidth));
      setOrientation(newHeight > newWidth ? "portrait" : "landscape");
    };

    // Set up event listener
    window.addEventListener("resize", handleResize, { passive: true });
    
    // Call once to set initial values
    handleResize();

    // Clean up
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return {
    device,
    width,
    height,
    orientation,
    isMobile,
    isTablet,
    isDesktop,
    isLargeDesktop,
    isMobileOrTablet,
    isAtLeast,
    isLessThan,
  };
}

export default useResponsive;
