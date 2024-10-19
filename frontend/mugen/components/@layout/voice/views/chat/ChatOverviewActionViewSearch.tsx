import { useState } from 'react'
import {
  Accordion,
  AccordionItem,
  Button,
  CardBody,
  CardHeader,
  Checkbox,
  CheckboxGroup,
  DateRangePicker,
  Dropdown,
  DropdownItem,
  DropdownMenu,
  DropdownTrigger,
  Input
} from '@nextui-org/react'

import { IconSearch } from '@/components/icons/Search'
import { CardPreseted } from '@/components/reusable/CardPreseted'
import { Dev } from '@/lib/const'
import { MessageText } from '@/lib/dto/voice/message'
import { ChatMessage } from './ChatMessage'

interface SearchResult {
  content: React.ReactNode
}

interface ChatOverviewActionViewSearchProps {}

const searchResults = [
  {
    content: (
      <ChatMessage
        messageId={Dev.MessageId1}
        senderUsername="dehwyy"
        senderImage={Dev.Img}
        time="10/14/2024 8:31 PM"
        component={new MessageText("it's a message")}
      />
    )
  },
  {
    content: (
      <ChatMessage
        messageId={Dev.MessageId2}
        senderUsername="dehwyy"
        senderImage={Dev.Img}
        time="10/14/2024 8:31 PM"
        component={new MessageText("it's a message 2")}
      />
    )
  }
]

export function ChatOverviewActionViewSearch(props: ChatOverviewActionViewSearchProps) {
  const [selectedKeys, setSelectedKeys] = useState<Set<string>>(new Set(['filter1']))

  const [inputValue, setInputValue] = useState('')
  return (
    <CardPreseted variant="gradient" className="pl-[calc(100%-350px-24px)] max-w-[374px]">
      <CardHeader>
        <div className="flex gap-x-2 items-center">
          <IconSearch />
          <p className="text-lg">Search</p>
        </div>
      </CardHeader>
      <CardBody className="overflow-x-hidden">
        <div className="w-[350px] flex flex-col gap-y-3 ">
          <Input placeholder="Search" value={inputValue} onValueChange={setInputValue} variant="underlined" />
          <section style={{ maxHeight: inputValue ? '200px' : '0px' }} className="flex flex-col gap-y-3 overflow-auto transition-all">
            {searchResults.map((item, i) => (
              <div key={i}>{item.content}</div>
            ))}
          </section>
          <Accordion>
            <AccordionItem title="Filters">
              <section className="flex flex-col gap-y-3">
                <div>
                  <CheckboxGroup color="secondary" label="Search among: ">
                    <Checkbox value="Messages">Messages</Checkbox>
                    <Checkbox value="Members">Members</Checkbox>
                  </CheckboxGroup>
                </div>

                <div className="select-none">
                  <DateRangePicker variant="faded" color="secondary" label="Date range:" className="max-w-xs" />
                </div>
              </section>
            </AccordionItem>
          </Accordion>
        </div>
      </CardBody>
    </CardPreseted>
  )
}
