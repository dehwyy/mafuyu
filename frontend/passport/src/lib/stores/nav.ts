import { localStorageStore } from '@skeletonlabs/skeleton'

export const enum CommunitySection {
  FOLLOWED = 'followed',
  FOLLOWERS = 'followers',
  FRIENDS = 'friends'
}

interface PersistentDataStore {
  communitySection: CommunitySection
}

export const persistentDataStore = localStorageStore<PersistentDataStore>(
  'community_data',
  {
    communitySection: CommunitySection.FRIENDS
  }
)

export const updatePersistentDataStore = (
  data: Partial<PersistentDataStore>
) => {
  persistentDataStore.update((v) => ({ ...v, ...data }))
}
