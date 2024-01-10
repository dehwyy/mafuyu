import { redirect } from '@sveltejs/kit';
import { Routes } from '$lib/const';

export function load() {
	throw redirect(302, Routes.Login)
}