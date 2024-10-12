import { ExpandableButtonGroup } from '../reusable/ExpandableButtonGroup'

const buttons = [
  { variant: 'shadow', color: 'secondary', content: 'Manage overlays' },
  { variant: 'shadow', color: 'primary', content: 'Add overlay' },
  { variant: 'shadow', color: 'danger', content: 'Clear overlays' }
]

const buttonHeight = 32
const buttonsGap = 10

export const OverlaysManagementMenu = () => {
  return (
    <ExpandableButtonGroup
      items={buttons}
      itemHeight={buttonHeight}
      gap={buttonsGap}
      className="mr-1 pt-2 mb-4"
    />
  )
}
