import { Button } from '@nextui-org/button'

const emojis =
  '😀 😃 😄 😁 😆 😅 😂 🤣 😊 😇 🙂 🙃 😉 😌 😍 🥰 😘 😗 😙 😚 😋 😛 😝 😜 🤪 🤨 🧐 🤓 😎 🤩 🥳 😏 😒 😞 😔 😟 😕 🙁 😣 😖 😫 😩 🥺 😢 😭 😮 💨 😤 😠 😡 🤬 🤯 😳 🥵 🥶 😱 😨 😰 😥 😓 🤗 🤔 🤭 🤫 🤥 😶 😶 😑 😬 🙄 😯 😦 😧 😮 😲 🥱 😴 🤤 😪 😵 😵 🤐 🥴 🤢 🤮 🤧 😷 🤒 🤕 🤑 🤠 😈 👿 👹 👺 🤡 💩 👻 👽 👾 🤖 🎃 😺 😸 😹 😻 😼 😽 🙀 😿 😾'

export const Emojis = () => {
  return (
    <div className="grid grid-cols-7 justify-between gap-y-3 px-1 h-[200px] max-h-[200px] overflow-y-auto">
      {emojis.split(' ').map((emoji) => (
        <Button
          key={emoji}
          isIconOnly
          disableAnimation
          className="bg-transparent"
        >
          <p className="text-3xl">{emoji}</p>
        </Button>
      ))}
    </div>
  )
}
