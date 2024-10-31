'use client';

import { useEffect, useState } from 'react';

import { Moon, Sun } from 'lucide-react';

export function ThemeToggle({ className = '' }: { className?: string }) {
  const [isDarkMode, setIsDarkMode] = useState(false);

  useEffect(() => {
    const cache = localStorage.getItem('theme');
    if (cache) {
      setIsDarkMode(cache === 'dark');
      return;
    }
    const root = window.document.documentElement;
    const initialColorValue = root.classList.contains('dark');
    setIsDarkMode(initialColorValue);
  }, []);

  useEffect(() => {
    if (isDarkMode) {
      document.documentElement.classList.add('dark');
      localStorage.setItem('theme', 'dark');
    } else {
      document.documentElement.classList.remove('dark');
      localStorage.setItem('theme', 'light');
    }
  }, [isDarkMode]);

  const toggleDarkMode = () => {
    setIsDarkMode(!isDarkMode);
  };

  return (
    <button onClick={toggleDarkMode} className={className}>
      {isDarkMode ? <Sun size={20} /> : <Moon size={20} />}
    </button>
  );
}
