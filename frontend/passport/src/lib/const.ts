export const enum Routes {
  Login = "/login",
  Create = "/create",
  RecoverPassword = "/recover/password",
  Account = "/",
  Logout = "/logout",
}

interface Language {
  language: string
  emoji_icon: string
}

export const MostPopularLanguage: Language[] = [
  { language: "Arabic", emoji_icon: "🇦🇪" },
  { language: "Dutch", emoji_icon: "🇩🇰" },
  { language: "English", emoji_icon: "🇺🇸" },
  { language: "French", emoji_icon: "🇫🇷" },
  { language: "German", emoji_icon: "🇩🇪" },
  { language: "Hindi", emoji_icon: "🇮🇳" },
  { language: "Indonesian", emoji_icon: "🇮🇩" },
  { language: "Italian", emoji_icon: "🇮🇹" },
  { language: "Japanese", emoji_icon: "🇯🇵" },
  { language: "Korean", emoji_icon: "🇰🇷" },
  { language: "Chinese", emoji_icon: "🇨🇳" },
  { language: "Polish", emoji_icon: "🇵🇱" },
  { language: "Portuguese", emoji_icon: "🇵🇹" },
  { language: "Russian", emoji_icon: "🇷🇺" },
  { language: "Spanish", emoji_icon: "🇪🇸" },
  { language: "Thai", emoji_icon: "🇹🇭" },
  { language: "Turkish", emoji_icon: "🇹🇷" },
]
