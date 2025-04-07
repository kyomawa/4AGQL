<template>
	<div class="w-full">
		<!-- Empty state -->
		<div v-if="grades.length === 0" class="py-8 text-center bg-white rounded-lg shadow">
			<svg xmlns="http://www.w3.org/2000/svg" class="w-16 h-16 mx-auto text-gray-400" fill="none" viewBox="0 0 24 24"
				stroke="currentColor">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
					d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
			</svg>
			<h3 class="mt-4 text-lg font-medium text-gray-900">No grades found</h3>
			<p class="mt-1 text-gray-500">{{ noGradesMessage }}</p>
		</div>

		<!-- Table state -->
		<div v-else class="overflow-x-auto bg-white rounded-lg shadow">
			<table class="min-w-full divide-y divide-gray-200">
				<thead class="bg-gray-50">
					<tr>
						<th v-if="showStudent" class="px-6 py-3 text-xs font-medium text-left text-gray-500 uppercase">Student</th>
						<th class="px-6 py-3 text-xs font-medium text-left text-gray-500 uppercase">Course</th>
						<th class="px-6 py-3 text-xs font-medium text-left text-gray-500 uppercase">Grade</th>
						<th v-if="showProfessor" class="px-6 py-3 text-xs font-medium text-left text-gray-500 uppercase">Professor
						</th>
						<th v-if="showActions" class="px-6 py-3 text-xs font-medium text-right text-gray-500 uppercase">Actions</th>
					</tr>
				</thead>
				<tbody class="bg-white divide-y divide-gray-200">
					<tr v-for="grade in grades" :key="grade.id" class="hover:bg-gray-50">
						<td v-if="showStudent" class="px-6 py-4">
							<div class="text-sm font-medium text-gray-900">{{ getStudentName(grade.user_id) }}</div>
							<div class="text-xs text-gray-500">ID: {{ grade.user_id }}</div>
						</td>
						<td class="px-6 py-4">{{ grade.course }}</td>
						<td class="px-6 py-4">
							<span :class="gradeColorClass(grade.value)"
								class="text-sm font-semibold px-2.5 py-0.5 rounded-full inline-flex items-center">
								{{ grade.value }}/20
							</span>
						</td>
						<td v-if="showProfessor" class="px-6 py-4">
							<div class="text-sm text-gray-900">{{ getProfessorName(grade.professor_id) }}</div>
							<div class="text-xs text-gray-500">ID: {{ grade.professor_id }}</div>
						</td>
						<td v-if="showActions" class="px-6 py-4 text-right">
							<slot name="actions" :grade="grade"></slot>
						</td>
					</tr>
				</tbody>
			</table>
		</div>

		<!-- Stats -->
		<div v-if="showStats && grades.length > 0" class="grid grid-cols-1 gap-6 mt-8 md:grid-cols-3">
			<div class="p-5 bg-white rounded-lg shadow">
				<h3 class="mb-2 text-lg font-semibold text-gray-700">Average Grade</h3>
				<p class="text-3xl font-bold" :class="gradeColorClass(averageGrade)">{{ averageGrade.toFixed(2) }}/20</p>
			</div>
			<div class="p-5 bg-white rounded-lg shadow">
				<h3 class="mb-2 text-lg font-semibold text-gray-700">Highest Grade</h3>
				<p class="text-3xl font-bold text-green-600">{{ highestGrade.toFixed(2) }}/20</p>
			</div>
			<div class="p-5 bg-white rounded-lg shadow">
				<h3 class="mb-2 text-lg font-semibold text-gray-700">Lowest Grade</h3>
				<p class="text-3xl font-bold text-red-600">{{ lowestGrade.toFixed(2) }}/20</p>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Grade } from '~/types/grade'
import type { User } from '~/types/user'

const props = defineProps<{
	grades: Grade[]
	students?: User[]
	professors?: User[]
	showStudent?: boolean
	showProfessor?: boolean
	showActions?: boolean
	showStats?: boolean
	noGradesMessage?: string
}>()

// Stats
const averageGrade = computed(() =>
	props.grades.length === 0 ? 0 : props.grades.reduce((acc, g) => acc + g.value, 0) / props.grades.length
)

const highestGrade = computed(() =>
	props.grades.length === 0 ? 0 : Math.max(...props.grades.map((g) => g.value))
)

const lowestGrade = computed(() =>
	props.grades.length === 0 ? 0 : Math.min(...props.grades.map((g) => g.value))
)

// Helpers
const gradeColorClass = (grade: number) => {
	if (grade >= 16) return 'text-green-600 bg-green-100'
	if (grade >= 12) return 'text-blue-600 bg-blue-100'
	if (grade >= 10) return 'text-yellow-600 bg-yellow-100'
	return 'text-red-600 bg-red-100'
}

const getStudentName = (id: string) => {
	const student = props.students?.find((s) => s.id === id)
	return student?.pseudo ?? id
}

const getProfessorName = (id: string) => {
	const professor = props.professors?.find((p) => p.id === id)
	return professor?.pseudo ?? id
}
</script>
