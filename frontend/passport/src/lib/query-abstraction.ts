import {
  createQuery,
  type UndefinedInitialDataOptions,
  type DefinedInitialDataOptions,
  type CreateQueryOptions,
  type CreateQueryResult,
} from "@tanstack/svelte-query"
import { derived, type Writable, writable } from "svelte/store"

type TQueryKey = ReadonlyArray<unknown>

type QueryOptions<Args, T, E, D, QueryKey extends TQueryKey = TQueryKey> = (args: Args) => UndefinedInitialDataOptions<T, E, D, QueryKey>

// Implemented only for `initialData` === `undefined` (It works like this with `svelte` reactivity)
const createReactiveQuery = <Args, T, E, D, QueryKey extends TQueryKey>(
  initialArguments: Args,
  queryOptions: QueryOptions<Args, T, E, D, QueryKey>,
) => {
  const store = writable<Args>(initialArguments)
  const query = createQuery<T, E, D, QueryKey>(derived(store, store_val => queryOptions(store_val))) as CreateQueryResult<T, Error>

  return [query, store] as const
}

export { createReactiveQuery }
