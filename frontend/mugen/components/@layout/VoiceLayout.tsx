import { Button } from '@nextui-org/button'
import { Aside, AsideScrollable, AsideTopPanel, UserBriefInformation } from '$layout/aside'
import { Content, ContentBottomPanel, ContentScrollable, ContentTopPanel } from '$layout/content'
import { AppShell } from '$layout/essential'

import { CurrentServer } from './voice/CurrentServer'
import { NavigationPanel } from './voice/NavigationPanel'

export function VoiceLayout() {
  return (
    <AppShell withHeader>
      <Aside>
        <AsideTopPanel>
          <div className="flex gap-x-3 h-full ">
            <Button
              className="h-full"
              variant="shadow"
              color="secondary"
            >
              Change layout
            </Button>
            <Button
              className="h-full"
              variant="shadow"
              color="secondary"
            >
              Manage overlays
            </Button>
          </div>
        </AsideTopPanel>
        <div className="flex flex-row gap-x-[4px] max-h-full min-h-full">
          <AsideScrollable>
            <NavigationPanel items={new Array(66).fill({})} />
          </AsideScrollable>
          <div className="flex-1">
            <AsideScrollable>
              <CurrentServer />
            </AsideScrollable>
          </div>
        </div>
      </Aside>
      <Content>
        <ContentTopPanel>some</ContentTopPanel>
        <ContentScrollable>some</ContentScrollable>
        <ContentBottomPanel>some</ContentBottomPanel>
      </Content>
    </AppShell>
  )
}
