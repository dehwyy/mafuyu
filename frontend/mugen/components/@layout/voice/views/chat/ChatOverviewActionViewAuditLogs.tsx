import { Avatar, CardBody, CardHeader, Divider } from '@nextui-org/react'

import { IconTerminal } from '@/components/icons/Terminal'
import { CardPreseted } from '@/components/reusable/CardPreseted'
import { Dev } from '@/lib/const'

interface ServerEvent {
  actor: string
  actorImage?: string
  action: string
  time: string
}

interface ChatOverviewActionViewAuditLogsProps {
  events: ServerEvent[]
}

export function ChatOverviewActionViewAuditLogs(props: ChatOverviewActionViewAuditLogsProps) {
  return (
    <CardPreseted variant="gradient">
      <CardHeader>
        <div className="flex gap-x-2 items-center">
          <IconTerminal />
          <p className="text-lg">Audit logs</p>
        </div>
      </CardHeader>
      <CardBody>
        <div className="w-[350px] flex flex-col gap-y-3">
          {props.events.map((ev, i) => (
            <>
              <div className="flex gap-x-3 items-center">
                <Avatar className="h-7 w-7" src={ev.actorImage || Dev.Img2} />
                <div>
                  <p className="flex gap-x-1">
                    <span className="font-semibold">{ev.actor}</span>
                    <span>{ev.action}</span>
                  </p>
                  <p className="text-[12px] text-default-500">{ev.time}</p>
                </div>
              </div>
              {i !== props.events.length - 1 && <Divider />}
            </>
          ))}
        </div>
      </CardBody>
    </CardPreseted>
  )
}
