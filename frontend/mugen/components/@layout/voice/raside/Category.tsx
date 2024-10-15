import { RAsideUser, RAsideUserProps } from './User'

interface CategoryProps {
  categoryName: string
  items: RAsideUserProps[]
}

export function Category(props: CategoryProps) {
  return (
    <article className="flex flex-col gap-y-1">
      <p className="text-[12px] font-semibold leading-4 text-default-400">{props.categoryName}</p>
      <div className="flex flex-col gap-y-0.5">
        {props.items.map((item, i) => (
          <RAsideUser key={i} {...item} />
        ))}
      </div>
    </article>
  )
}
