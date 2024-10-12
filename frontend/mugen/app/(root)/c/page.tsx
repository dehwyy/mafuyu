'use client'

import { useState } from 'react'

import { ChatLayout } from '@/components/@layout/ChatLayout'
import { VoiceLayout } from '@/components/@layout/VoiceLayout'

type Layout = 'default' | 'chat' | 'user'
export default function $Page() {
  const [currentLayout, setCurrentLayout] = useState()
  return <VoiceLayout />
}
