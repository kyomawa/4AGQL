<template>
	<div class="w-[830px] mx-auto mt-16 text-center">
		<Head>
			<title>Accueil - SchoolInc</title>
		</Head>

		<!-- Titre avec animation de fade-in -->
		<h1 class="title text-animation">
			N'attendez plus,
			<span class="text-violet-600 dark:text-violet-300">SchoolInc est là pour vous.</span>
		</h1>

		<!-- Sous-titre avec animation de fade-in -->
		<p class="subtitle subtitle-animation">
			Simplifiez le suivi de vos cours et de vos notes en un seul endroit.
		</p>

		<!-- Carte décorative avec animation -->
		<div class="mt-16 card-animation">
			<div class="w-full p-6 bg-white card dark:bg-violet-950">
				<div class="flex flex-col items-center md:flex-row md:justify-around">
					<div class="feature-item">
						<div
							class="flex items-center justify-center w-12 h-12 mb-4 text-white bg-violet-600 rounded-full"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-6 h-6"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
								/>
							</svg>
						</div>
						<h3 class="mb-2 text-lg font-semibold text-violet-700 dark:text-violet-300">
							Gestion simplifiée
						</h3>
						<p class="text-gray-600 dark:text-violet-200">
							Organisez vos cours et vos devoirs en quelques clics
						</p>
					</div>

					<div class="feature-item mt-8 md:mt-0">
						<div
							class="flex items-center justify-center w-12 h-12 mb-4 text-white bg-violet-600 rounded-full"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-6 h-6"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
								/>
							</svg>
						</div>
						<h3 class="mb-2 text-lg font-semibold text-violet-700 dark:text-violet-300">
							Suivi des notes
						</h3>
						<p class="text-gray-600 dark:text-violet-200">
							Consultez vos résultats et visualisez votre progression
						</p>
					</div>

					<div class="feature-item mt-8 md:mt-0">
						<div
							class="flex items-center justify-center w-12 h-12 mb-4 text-white bg-violet-600 rounded-full"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-6 h-6"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
								/>
							</svg>
						</div>
						<h3 class="mb-2 text-lg font-semibold text-violet-700 dark:text-violet-300">
							Temps réel
						</h3>
						<p class="text-gray-600 dark:text-violet-200">
							Accédez instantanément aux mises à jour de vos cours et notes
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
// page d'accueil
</script>

<style scoped>
.title {
	@apply text-5xl font-bold text-violet-700 dark:text-violet-200;
}

.subtitle {
	@apply mt-4 text-xl text-gray-600 dark:text-violet-300;
}

.card {
	@apply shadow-lg rounded-2xl transition duration-300;
}

.cta-button {
	@apply px-8 py-4 text-lg font-medium text-white transition-colors bg-violet-600 rounded-lg shadow-lg hover:bg-violet-700 transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500;
}

.btn-primary {
	@apply px-6 py-3 font-medium text-white transition-colors bg-violet-600 rounded-lg hover:bg-violet-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500;
}

.btn-secondary {
	@apply px-6 py-3 font-medium transition-colors border-2 border-violet-600 text-violet-700 dark:text-violet-300 dark:border-violet-400 rounded-lg hover:bg-violet-50 dark:hover:bg-violet-900 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500;
}

.feature-item {
	@apply flex flex-col items-center max-w-xs px-4 py-2 text-center;
}

/* Animations */
.logo-animation {
	animation:
		fadeInDown 1.2s ease-out,
		bounce 2s ease-in-out infinite alternate;
	animation-delay: 0s, 1.2s;
}

.logo-pulse {
	border-radius: 50%;
	box-shadow: 0 0 0 0 rgba(124, 58, 237, 0.7);
	animation: pulse 2s infinite;
	animation-delay: 1s;
}

.text-animation {
	opacity: 0;
	animation: fadeIn 1s ease-out forwards;
	animation-delay: 0.7s;
}

.subtitle-animation {
	opacity: 0;
	animation: fadeIn 1s ease-out forwards;
	animation-delay: 1.2s;
}

.cta-animation {
	opacity: 0;
	animation:
		fadeInUp 1s ease-out forwards,
		pulse-shadow 3s infinite;
	animation-delay: 1.5s, 2.5s;
}

.buttons-animation {
	opacity: 0;
	animation: fadeInUp 1s ease-out forwards;
	animation-delay: 1.7s;
}

.card-animation {
	opacity: 0;
	animation: fadeInUp 1s ease-out forwards;
	animation-delay: 1.9s;
}

.gradient-text {
	@apply text-transparent bg-clip-text;
	background-image: linear-gradient(to right, #8b5cf6, #ec4899);
}

@keyframes fadeIn {
	from {
		opacity: 0;
	}
	to {
		opacity: 1;
	}
}

@keyframes fadeInDown {
	from {
		opacity: 0;
		transform: translateY(-30px);
	}
	to {
		opacity: 1;
		transform: translateY(0);
	}
}

@keyframes fadeInUp {
	from {
		opacity: 0;
		transform: translateY(20px);
	}
	to {
		opacity: 1;
		transform: translateY(0);
	}
}

@keyframes bounce {
	from {
		transform: translateY(0);
	}
	to {
		transform: translateY(-10px);
	}
}

@keyframes pulse {
	0% {
		box-shadow: 0 0 0 0 rgba(124, 58, 237, 0.7);
	}
	70% {
		box-shadow: 0 0 0 20px rgba(124, 58, 237, 0);
	}
	100% {
		box-shadow: 0 0 0 0 rgba(124, 58, 237, 0);
	}
}

@keyframes pulse-shadow {
	0% {
		box-shadow: 0 0 0 0 rgba(124, 58, 237, 0.3);
	}
	70% {
		box-shadow: 0 0 10px 5px rgba(124, 58, 237, 0.2);
	}
	100% {
		box-shadow: 0 0 0 0 rgba(124, 58, 237, 0);
	}
}
</style>
