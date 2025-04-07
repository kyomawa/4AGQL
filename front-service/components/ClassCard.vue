<template>
	<div class="overflow-hidden transition bg-white rounded-lg shadow hover:shadow-lg">
		<div class="p-5 border-b">
			<h3 class="text-xl font-semibold text-blue-700">{{ classItem.name }}</h3>
		</div>
		<div class="p-5">
			<div class="flex items-center mb-3">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="w-5 h-5 mr-2 text-gray-500"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
					/>
				</svg>
				<span class="text-gray-600">
					{{ classItem.students ? classItem.students.length : 0 }} Students
				</span>
			</div>
			<div class="flex items-center mb-4">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="w-5 h-5 mr-2 text-gray-500"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path d="M12 14l9-5-9-5-9 5 9 5z" />
					<path
						d="M12 14l6.16-3.422a12.083 12.083 0 01.665 6.479A11.952 11.952 0 0012 20.055a11.952 11.952 0 00-6.824-2.998 12.078 12.078 0 01.665-6.479L12 14z"
					/>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 14l9-5-9-5-9 5 9 5zm0 0l6.16-3.422a12.083 12.083 0 01.665 6.479A11.952 11.952 0 0012 20.055a11.952 11.952 0 00-6.824-2.998 12.078 12.078 0 01.665-6.479L12 14zm-4 6v-7.5l4-2.222"
					/>
				</svg>
				<span class="text-gray-600">
					{{ classItem.professors ? classItem.professors.length : 0 }} Professors
				</span>
			</div>
			<NuxtLink
				:to="`/classes/${classItem.id}`"
				class="block py-2 text-center text-white transition bg-blue-600 rounded-md hover:bg-blue-700"
			>
				View Details
			</NuxtLink>
		</div>
	</div>
</template>

<script setup lang="ts">
defineProps({
	classItem: {
		type: Object,
		required: true,
	},
});
</script>
