import React, { useMemo } from 'react'
import { Group } from '@visx/group'
import { scaleBand, scaleLinear } from '@visx/scale'
import { Bar } from '@visx/shape'

interface LetterFrequency {
  letter: string
  frequency: number
}

const data: LetterFrequency[] = [
  { letter: 's', frequency: 0.14 },
  { letter: 'a', frequency: 0.04 },
  { letter: 't', frequency: 0.02 },
  { letter: 'e', frequency: 0.02 },
  { letter: 'r', frequency: 0.02 },
  { letter: 'o', frequency: 0.02 }
]

const getLetter = (d: LetterFrequency) => d.letter
const getLetterFrequency = (d: LetterFrequency) => Number(d.frequency) * 100

interface OnlineStatsProps {
  width: number
  height: number
  my?: number
}

export default function OnlineStats({ width, height, my = 0 }: OnlineStatsProps) {
  // bounds
  const xMax = width
  const yMax = height - my

  // scales, memoize for performance
  const xScale = useMemo(
    () =>
      scaleBand<string>({
        range: [0, xMax],
        round: true,
        domain: data.map(getLetter),
        padding: 0.4
      }),
    [xMax]
  )
  const yScale = useMemo(
    () =>
      scaleLinear<number>({
        range: [yMax, 0],
        round: true,
        domain: [0, Math.max(...data.map(getLetterFrequency))]
      }),
    [yMax]
  )

  return width < 10 ? null : (
    <svg
      width={width}
      height={height}
    >
      <rect
        width={width}
        height={height}
        fill="none"
      />
      <Group top={my / 2}>
        {data.map((d) => {
          const letter = getLetter(d)
          const barWidth = xScale.bandwidth()
          const barHeight = yMax - (yScale(getLetterFrequency(d)) ?? 0)
          const barX = xScale(letter)
          const barY = yMax - barHeight
          return (
            <Bar
              key={`bar-${letter}`}
              x={barX}
              y={barY}
              width={barWidth}
              height={barHeight}
              fill="#7FFF00"
              rx={3}
            />
          )
        })}
      </Group>
    </svg>
  )
}
