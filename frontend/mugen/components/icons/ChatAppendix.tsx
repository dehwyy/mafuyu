import clsx from 'clsx'

interface IconChatAppendixProps {
  className?: string
}

export const ChatAppendix = ({ className }: IconChatAppendixProps) => {
  return (
    <svg
      width="9"
      height="17"
    >
      <g
        fill="none"
        fill-rule="evenodd"
      >
        <path
          d="M3 17h6V0c-.193 2.84-.876 5.767-2.05 8.782-.904 2.325-2.446 4.485-4.625 6.48A1 1 0 003 17z"
          fill="#000"
          filter="url(#messageAppendix)"
        ></path>
        <path
          d="M3 17h6V0c-.193 2.84-.876 5.767-2.05 8.782-.904 2.325-2.446 4.485-4.625 6.48A1 1 0 003 17z"
          fill="FFF"
          stroke=""
          className={className}
        ></path>
      </g>
    </svg>
  )
}
