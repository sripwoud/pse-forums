// This file was generated by [rspc](https://github.com/specta-rs/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "comment.list", input: number, result: CommentDto[] } | 
        { key: "comment.read", input: number, result: CommentDto } | 
        { key: "group.list", input: never, result: GroupDto[] } | 
        { key: "group.read", input: number, result: GroupDto } | 
        { key: "post.list", input: number, result: PostDto[] } | 
        { key: "post.read", input: number, result: PostDto } | 
        { key: "profile.list", input: never, result: ProfileDto[] } | 
        { key: "profile.read", input: string, result: ProfileDto } | 
        { key: "user.read", input: string, result: UserDto },
    mutations: 
        { key: "auth.signin", input: SigninRequestDto, result: AuthResponseDto } | 
        { key: "auth.signup", input: SignupRequestDto, result: AuthResponseDto } | 
        { key: "comment.create", input: CreateCommentDto, result: CommentDto } | 
        { key: "comment.delete", input: number, result: CommentDto } | 
        { key: "comment.update", input: UpdateCommentDto, result: CommentDto } | 
        { key: "group.create", input: CreateGroupDto, result: GroupDto } | 
        { key: "group.delete", input: number, result: GroupDto } | 
        { key: "group.update", input: UpdateGroupDto, result: GroupDto } | 
        { key: "post.create", input: CreatePostDto, result: PostDto } | 
        { key: "post.delete", input: number, result: PostDto } | 
        { key: "post.update", input: UpdatePostDto, result: PostDto } | 
        { key: "profile.update", input: UpdateProfileReqDto, result: UpdateProfileResDto } | 
        { key: "user.delete", input: string, result: UserDto },
    subscriptions: never
};

export type AuthResponseDto = { user: UserDto; token: string }

export type CommentDto = { created_at: string; content: string; id: number; pid: number; rid: number | null; uid: number | null }

export type CreateCommentDto = { content: string; pid: number; rid: number | null; uid: number | null }

export type CreateGroupDto = { create_bandada_group: boolean | null; description: string; name: string; tags: string[] | null }

export type CreatePostDto = { content: string; gid: number | null; tags: string[] | null; title: string; uid: number | null }

export type GroupDto = { bandada_admin_id: number | null; id: number; name: string; description: string; tags: string[] }

export type PostDto = { created_at: string; content: string; gid: number | null; id: number; tags: string[]; title: string; uid: number | null }

export type ProfileDto = { id: number; about: string | null; created_at: string; username: string; url: string | null }

export type SigninRequestDto = { username: string; password: string }

export type SignupRequestDto = { email: string; password: string; username: string }

export type UpdateCommentDto = { content: string; id: number }

export type UpdateGroupDto = { description: string | null; id: number; name: string | null; tags: string[] | null }

export type UpdatePostDto = { content: string | null; id: number; tags: string[] | null; title: string | null }

export type UpdateProfileReqDto = { about: string | null; id: number; url: string | null; username: string | null }

export type UpdateProfileResDto = { profile: ProfileDto; jwt: string | null }

export type UserDto = { created_at: string; id: number; email: string; email_confirmed: boolean; username: string }
