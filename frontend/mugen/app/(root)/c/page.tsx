'use client'

import { useState } from 'react'

import { ChatLayout } from '@/components/@layout/ChatLayout'
import { Voice2Layout } from '@/components/@layout/Voice2Layout'
import { VoiceLayout } from '@/components/@layout/VoiceLayout'

type Layout = 'default' | 'chat' | 'user'
export default function $Page() {
  const [currentLayout, setCurrentLayout] = useState()
  return <Voice2Layout />
}
