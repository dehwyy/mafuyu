import { redirect } from '@sveltejs/kit';
import { Routes } from '$lib/const';
import type {PageServerLoad} from "./$types"

export const load: PageServerLoad = async ({ parent}) => {

	const data = await parent()
	if (data.username) {
		throw redirect(307, Routes.Account + `/@${data.username}`)
	} else {
		throw redirect(302, Routes.Login)
	}
}