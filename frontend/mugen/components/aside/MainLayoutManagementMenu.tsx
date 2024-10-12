import { ExpandableButtonGroup } from '../reusable/ExpandableButtonGroup'

const buttons = [
  { variant: 'shadow', color: 'secondary', content: 'Current layout' },
  { variant: 'shadow', color: 'primary', content: 'Change' },
  { variant: 'shadow', color: 'danger', content: 'Reset' }
]

const buttonHeight = 32
const buttonsGap = 10

export const MainLayoutManagementMenu = () => {
  return (
    <ExpandableButtonGroup
      items={buttons}
      itemHeight={buttonHeight}
      gap={buttonsGap}
      className="mr-1"
    />
  )
}
