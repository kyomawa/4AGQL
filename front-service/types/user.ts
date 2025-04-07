// types/user.ts
export interface User {
	id: string;
	email: string;
	pseudo: string;
	role: string;
}

export interface ExternalClaim {
	id: string;
	role: string;
}

export interface Grade {
	id: string;
	user_id: string;
	course: string;
	value: number;
	professor_id: string;
}

export interface Class {
	id: string;
	name: string;
	students: string[];
	professors: string[];
}
