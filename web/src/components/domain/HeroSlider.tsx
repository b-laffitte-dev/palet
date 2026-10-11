import React, { useState, useEffect } from "react";
import { Match } from "../../types";
import { MatchHero, MatchHeroProps } from "./MatchHero";

export interface HeroSliderProps {
  matches: Match[];
  autoPlay?: boolean;
  interval?: number;
}

/**
 * HeroSlider - Carousel of featured matches for the home page
 * Top 14 inspired slider component
 */
export function HeroSlider({ matches, autoPlay = true, interval = 8000 }: HeroSliderProps) {
  const [currentIndex, setCurrentIndex] = useState(0);

  // Auto-advance the slider
  useEffect(() => {
    if (autoPlay && matches.length > 1) {
      const timer = setInterval(() => {
        setCurrentIndex((prevIndex) => (prevIndex + 1) % matches.length);
      }, interval);
      return () => clearInterval(timer);
    }
  }, [autoPlay, interval, matches.length]);

  if (matches.length === 0) return null;

  return (
    <div className="hero-slider relative w-full overflow-hidden rounded-[var(--radius-xl)] shadow-2xl">
      {/* Slider Container */}
      <div
        className="slider-track flex transition-transform duration-500 ease-in-out"
        style={{ transform: `translateX(-${currentIndex * 100}%)` }}
      >
        {matches.map((match) => (
          <div
            key={match.id}
            className="slider-slide min-w-full p-[var(--spacing-4)] lg:p-[var(--spacing-8)]"
          >
            <MatchHero match={match} />
          </div>
        ))}
      </div>

      {/* Navigation Dots */}
      {matches.length > 1 && (
        <div className="absolute bottom-[var(--spacing-4)] left-[var(--spacing-4)] right-[var(--spacing-4)] flex justify-center gap-[var(--spacing-2)]">
          {matches.map((_, index) => (
            <button
              key={index}
              onClick={() => setCurrentIndex(index)}
              className={`w-2 h-2 rounded-full transition-all duration-200 ${
                index === currentIndex
                  ? "bg-[var(--color-gold)] w-4"
                  : "bg-white/30 hover:bg-white/50"
              }`}
              aria-label={`Aller au diapositif ${index + 1}`}
            />
          ))}
        </div>
      )}

      {/* Navigation Arrows */}
      {matches.length > 1 && (
        <>
          <button
            onClick={() =>
              setCurrentIndex((prevIndex) => (prevIndex - 1 + matches.length) % matches.length)
            }
            className="absolute left-[var(--spacing-2)] top-1/2 -translate-y-1/2 bg-black/50 hover:bg-black/70 text-white w-10 h-10 rounded-full flex items-center justify-center transition-all duration-200 z-10"
            aria-label="Diapositif précédent"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 19l-7-7 7-7"
              />
            </svg>
          </button>
          <button
            onClick={() => setCurrentIndex((prevIndex) => (prevIndex + 1) % matches.length)}
            className="absolute right-[var(--spacing-2)] top-1/2 -translate-y-1/2 bg-black/50 hover:bg-black/70 text-white w-10 h-10 rounded-full flex items-center justify-center transition-all duration-200 z-10"
            aria-label="Diapositif suivant"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
            </svg>
          </button>
        </>
      )}
    </div>
  );
}

export default HeroSlider;
