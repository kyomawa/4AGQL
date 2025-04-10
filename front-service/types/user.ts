export interface User {
	id: string;
	email: string;
	pseudo: string;
	firstName?: string;
	lastName?: string;
	createdAt?: string;
	updatedAt?: string;
	classIds: string[];
	role: string;
}
