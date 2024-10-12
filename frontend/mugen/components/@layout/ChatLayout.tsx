import { useState } from 'react'
import { Aside, AsideScrollable, AsideTopPanel, UserBriefInformation } from '$layout/aside'
import { Content, ContentBottomPanel, ContentScrollable, ContentTopPanel } from '$layout/content'
import { AppShell } from '$layout/essential'

import { ChatInput, ChatList, ChatListItem, CurrentChatInformation, Message } from '@/components/@layout/chat'
import { Dev } from '@/lib/const'

export function ChatLayout() {
  const [focused, setFocused] = useState(0)

  return (
    <AppShell withHeader>
      <Aside>
        <AsideTopPanel>
          <UserBriefInformation
            name="dehwyy"
            description="awesome user"
            avatarSrc={Dev.Img}
          />
        </AsideTopPanel>
        <AsideScrollable>
          <ChatList>
            {new Array(20).fill(0).map((_, i) => (
              <ChatListItem
                key={i}
                name="dehwyy"
                avatarSrc={Dev.Img}
                description="ИУ5"
                lastAction="#believer"
                lastActionTime="20:31"
                isFocused={focused === i}
                onClickAction={() => {
                  setFocused(i)
                }}
                notificationsCount={i}
              />
            ))}
          </ChatList>
        </AsideScrollable>
      </Aside>
      <Content>
        <ContentTopPanel>
          <CurrentChatInformation
            name="dehwyy"
            avatarSrc={Dev.Img}
          />
        </ContentTopPanel>
        <ContentScrollable>
          {new Array(20).fill(0).map((_, i) => (
            <Message
              key={i}
              currentUserId="user_true"
              senderUserId={'user_' + String(i % 4 == 0)}
              senderUsername="dehwyy"
              senderUserImage={Dev.Img}
              nextMessageSenderUserId={'user_' + String((i + 1) % 4 == 0 || i == 19)}
              messageType="plain/text"
              messageContent={"It's a message!"}
              messageTimestamp="20:31"
            />
          ))}
        </ContentScrollable>
        <ContentBottomPanel>
          <ChatInput />
        </ContentBottomPanel>
      </Content>
    </AppShell>
  )
}
