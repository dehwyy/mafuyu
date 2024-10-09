'use client'

import { Accordion, AccordionItem, Button } from '@nextui-org/react'

export default function Home() {
  const defaultContent =
    'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.'

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <Button color="primary">It's a button!</Button>
      <Accordion>
        <AccordionItem
          key="1"
          aria-label="Accordion 1"
          title="Accordion 1"
        >
          {defaultContent}
        </AccordionItem>
        <AccordionItem
          key="2"
          aria-label="Accordion 2"
          title="Accordion 2"
        >
          {defaultContent}
        </AccordionItem>
        <AccordionItem
          key="3"
          aria-label="Accordion 3"
          title="Accordion 3"
        >
          {defaultContent}
        </AccordionItem>
      </Accordion>
    </main>
  )
}
