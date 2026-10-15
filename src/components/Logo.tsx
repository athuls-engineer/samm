import React from 'react';

interface LogoProps {
  className?: string;
  size?: number;
}

export const Logo: React.FC<LogoProps> = ({ className = 'w-9 h-9', size }) => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 128 128"
      className={className}
      width={size}
      height={size}
      fill="none"
    >
      <defs>
        <linearGradient id="sammBgGrad" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stopColor="#1e222b" />
          <stop offset="100%" stopColor="#0d0f14" />
        </linearGradient>
        <linearGradient id="sammShieldGrad" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stopColor="#fbbf24" />
          <stop offset="50%" stopColor="#f59e0b" />
          <stop offset="100%" stopColor="#b45309" />
        </linearGradient>
        <linearGradient id="sammInnerGrad" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" stopColor="#272c38" />
          <stop offset="100%" stopColor="#13161d" />
        </linearGradient>
      </defs>

      {/* Rounded Background Plate */}
      <rect x="4" y="4" width="120" height="120" rx="26" fill="url(#sammBgGrad)" stroke="#374151" strokeWidth="2.5" />

      {/* Golden Outer Shield */}
      <path
        d="M 64 16 L 104 32 C 104 74 64 108 64 108 C 64 108 24 74 24 32 Z"
        fill="none"
        stroke="url(#sammShieldGrad)"
        strokeWidth="4.5"
        strokeLinejoin="round"
      />

      {/* Inner Shield Fill */}
      <path
        d="M 64 22 L 98 36 C 98 70 64 99 64 99 C 64 99 30 70 30 36 Z"
        fill="url(#sammInnerGrad)"
        stroke="#475569"
        strokeWidth="1.5"
      />

      {/* Gothic SA Letters */}
      <path
        d="M 58 42 C 45 42 41 46 41 51 C 41 57 47 59 55 61 C 65 63 71 66 71 74 C 71 83 63 87 51 87 C 42 87 36 84 33 80 L 37 73 C 40 76 45 79 51 79 C 58 79 62 76 62 72 C 62 67 56 65 48 63 C 39 61 32 57 32 49 C 32 42 39 36 51 36 C 59 36 64 38 68 41 L 64 48 C 60 45 56 42 58 42 Z"
        fill="#f8fafc"
      />

      <path
        d="M 78 37 L 93 86 L 84 86 L 80 72 L 67 72 L 67 65 L 78 65 L 75 51 L 70 65 L 67 65 L 74 37 Z"
        fill="url(#sammShieldGrad)"
      />

      {/* Modding Gear Accent */}
      <circle cx="64" cy="92" r="5" fill="#f59e0b" />
      <circle cx="64" cy="92" r="2.2" fill="#13161d" />
    </svg>
  );
};
