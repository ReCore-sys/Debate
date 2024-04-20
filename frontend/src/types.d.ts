export interface LoginRequest { email: string, password: string, }
export interface Message { author: string, content: string, timestamp: string, channel: string, }
export interface StatusResponse { db: boolean, }
export interface User { username: string, password: string, uuid: string, pfp: string, email: string, bio: string, }
