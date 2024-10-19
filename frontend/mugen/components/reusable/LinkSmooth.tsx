import { Link, LinkProps } from '@nextui-org/link';





interface LinkSmoothProps extends LinkProps {
  anchorId: string
  children: React.ReactNode
}

export function LinkSmooth(props: LinkSmoothProps) {
  return (
    <Link
      onClick={(e) => {
        e.preventDefault()
        document.getElementById(props.anchorId)?.scrollIntoView({ behavior: 'smooth' })
        props.onClick?.(e)
      }}
      href={'#' + props.anchorId}
      {...props}
    >
      {props.children}
    </Link>
  )
}
