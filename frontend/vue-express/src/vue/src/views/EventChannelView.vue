<script setup lang="ts">
import { onUnmounted, onMounted, ref, reactive } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import NavBar from '../components/NavBar.vue'
import { showConfirmationPopup, showErrorPopup } from '../utils/popups'
import { isLoggedIn } from '../utils/auth'

interface User {
  id: string
  username: string
  fullName?: string
  avatarUrl?: string
}

interface Comment {
  id: string
  lastUpdatedAt: string
  content?: string
  imageUrl?: string
  author?: User
  isEditing?: boolean
  editContent?: string
  editImage?: number[] | null
  editImagePreview?: string | null
  isMenuOpen?: boolean
}

interface Post {
  id: string
  lastUpdatedAt: string
  title: string
  content: string
  imageUrl?: string
  reactionCount: number
  commentCount: number
  author?: User
  isReactedByActor: boolean
  displayComments: Comment[]
  areCommentsLoaded: boolean
  isMenuOpen?: boolean
}

const route = useRoute()
const router = useRouter()
const eventId = `urn:uuid:${route.params.id as string}`

const actorUserId = ref<string>('')
const eventName = ref<string>('Loading Event...')
const posts = ref<Post[]>([])
const isLoading = ref(true)

const userCache = reactive<Map<string, { fullName: string; avatarUrl: string }>>(new Map())

const showCreateModal = ref(false)
const isSubmitting = ref(false)
const editingPostId = ref<string | null>(null)

const newPost = reactive({
  title: '',
  content: '',
  image: null as number[] | null,
  imagePreview: null as string | null
})

interface CommentInputState {
  text: string
  image: number[] | null
  imagePreview: string | null
}
const commentInputs = reactive<Record<string, CommentInputState>>({})

const ensureInputState = (postId: string): CommentInputState => {
  if (!commentInputs[postId]) {
    commentInputs[postId] = { text: '', image: null, imagePreview: null }
  }
  return commentInputs[postId]!
}

const formatDate = (dateStr: string) => {
  return (
    new Date(dateStr).toLocaleDateString() +
    ' ' +
    new Date(dateStr).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
  )
}

const isOwner = (author?: User) => {
  if (!author || !actorUserId.value) return false
  return author.id === actorUserId.value
}

const fetchUserDetails = async (userId: string) => {
  if (userCache.has(userId)) return userCache.get(userId)

  try {
    console.log(userId)
    const response = await fetch(`http://localhost:4000/api/users/${userId}`, {
      method: 'GET',
      credentials: 'include'
    })

    if (response.ok) {
      const data = await response.json()
      const details = {
        fullName: data.fullname || data.username,
        avatarUrl: data.avatar_url
      }
      userCache.set(userId, details)
      return details
    }
  } catch (e) {
    console.error(`Failed to fetch user details for ${userId}`, e)
  }
  return null
}

const enrichAuthor = async (author?: User) => {
  if (!author || !author.id) return
  const details = await fetchUserDetails(author.id)
  if (details) {
    author.fullName = details.fullName
    author.avatarUrl = details.avatarUrl
  }
}

const fetchEventDetails = async () => {
  try {
    const response = await fetch(`http://localhost:4000/api/volunteer/events/urn:uuid:${route.params.id}`, {
      method: 'GET',
      credentials: 'include'
    })
    if (response.ok) {
      const data = await response.json()
      eventName.value = data.name
    }
  } catch (e) {
    console.error('Failed to fetch event details', e)
    eventName.value = 'Event Discussion'
  }
}

const fetchPosts = async () => {
  try {
    const response = await fetch(`http://localhost:4000/api/events/${eventId}/channel`, {
      method: 'GET',
      credentials: 'include'
    })

    if (response.ok) {
      const data = await response.json()

      const mappedPosts = data.map((p: any) => {
        ensureInputState(p.id)
        return {
          ...p,
          displayComments: p.commentsByActor || [],
          areCommentsLoaded: false,
          isMenuOpen: false
        }
      })

      await Promise.all(mappedPosts.map((post: Post) => enrichAuthor(post.author)))
      posts.value = mappedPosts
    } else {
      const err = await response.json()
      if (err.error === 'UserUnauthorized') {
        showErrorPopup('Fetching Posts', 'You must be a volunteer or manager to discuss events', 100)
        router.push('/home')
      }
    }
  } catch (error) {
    console.error(error)
  } finally {
    isLoading.value = false
  }
}

const loadPostDetails = async (post: Post) => {
  try {
    const response = await fetch(`http://localhost:4000/api/posts/${post.id}`, {
      method: 'GET',
      credentials: 'include'
    })
    if (response.ok) {
      const data = await response.json()
      const mappedComments = data.comments.map((c: any) => ({
        ...c,
        isEditing: false,
        isMenuOpen: false
      }))
      await Promise.all(mappedComments.map((c: Comment) => enrichAuthor(c.author)))
      post.displayComments = mappedComments
      post.commentCount = post.displayComments.length
      post.areCommentsLoaded = true
    }
  } catch (e) {
    console.error('Failed to load comments', e)
  }
}

const handleFileUpload = async (
  event: Event,
  target: 'post' | 'comment' | 'comment-edit',
  contextId?: string,
  commentObj?: Comment
) => {
  const input = event.target as HTMLInputElement
  if (input.files && input.files[0]) {
    const file = input.files[0]
    const buffer = await file.arrayBuffer()
    const bytes = Array.from(new Uint8Array(buffer))

    if (target === 'post') {
      newPost.image = bytes
      newPost.imagePreview = URL.createObjectURL(file)
    } else if (target === 'comment' && contextId) {
      const state = ensureInputState(contextId)
      state.image = bytes
      state.imagePreview = URL.createObjectURL(file)
      input.value = ''
    } else if (target === 'comment-edit' && commentObj) {
      commentObj.editImage = bytes
      commentObj.editImagePreview = URL.createObjectURL(file)
      input.value = ''
    }
  }
}

const openCreateModal = () => {
  editingPostId.value = null
  newPost.title = ''
  newPost.content = ''
  newPost.image = null
  newPost.imagePreview = null
  showCreateModal.value = true
}

const openEditPostModal = (post: Post) => {
  editingPostId.value = post.id
  newPost.title = post.title
  newPost.content = post.content
  newPost.image = null
  newPost.imagePreview = post.imageUrl || null
  showCreateModal.value = true
}

const handlePostSubmit = async () => {
  if (editingPostId.value) {
    await updatePost()
  } else {
    await createPost()
  }
}

const createPost = async () => {
  if (!newPost.title || !newPost.content) {
    showErrorPopup('Error', 'Title and Content are required')
    return
  }

  isSubmitting.value = true
  try {
    const response = await fetch(`http://localhost:4000/api/events/${eventId}/posts`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({
        postTitle: newPost.title,
        postContent: newPost.content,
        postImage: newPost.image
      })
    })

    if (response.status === 201) {
      showCreateModal.value = false
      await fetchPosts()
    } else {
      showErrorPopup('Error', 'Failed to create post')
    }
  } catch (e) {
    showErrorPopup('Error', 'Network error')
  } finally {
    isSubmitting.value = false
  }
}

const updatePost = async () => {
  if (!newPost.title || !newPost.content || !editingPostId.value) return

  isSubmitting.value = true
  try {
    const response = await fetch(`http://localhost:4000/api/posts/${editingPostId.value}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({
        postTitle: newPost.title,
        postContent: newPost.content,
        postImage: newPost.image
      })
    })

    if (response.ok) {
      showCreateModal.value = false
      const postIndex = posts.value.findIndex((p) => p.id === editingPostId.value)
      const postToUpdate = posts.value[postIndex]
      if (postIndex !== -1 && postToUpdate) {
        postToUpdate.title = newPost.title
        postToUpdate.content = newPost.content
        await fetchPosts()
      }
    } else {
      showErrorPopup('Error', 'Failed to update post')
    }
  } catch (e) {
    showErrorPopup('Error', 'Network error')
  } finally {
    isSubmitting.value = false
  }
}

const deletePost = async (postId: string) => {
  if (!(await showConfirmationPopup('Delete Post', 'Are you sure?'))) return

  try {
    const response = await fetch(`http://localhost:4000/api/posts/${postId}`, {
      method: 'DELETE',
      credentials: 'include'
    })
    if (response.ok) {
      posts.value = posts.value.filter((p) => p.id !== postId)
      delete commentInputs[postId]
    } else {
      showErrorPopup('Error', 'Could not delete post')
    }
  } catch (e) {
    console.error(e)
  }
}

const postComment = (post: Post) => {
  const inputState = ensureInputState(post.id)

  if (!inputState.text && !inputState.image) return

  const payload = {
    commentContent: inputState.text,
    commentImage: inputState.image || undefined
  }

  const savedPreview = inputState.imagePreview

  inputState.text = ''
  inputState.image = null
  inputState.imagePreview = null

  setTimeout(async () => {
    try {
      const response = await fetch(`http://localhost:4000/api/posts/${post.id}/comments`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify(payload)
      })

      if (response.status === 201) {
        await loadPostDetails(post)
      } else {
        throw new Error('Server returned error')
      }
    } catch (e) {
      console.error(e)
      inputState.text = payload.commentContent
      inputState.image = payload.commentImage || null
      inputState.imagePreview = savedPreview
      showErrorPopup('Delivery Failed', 'Could not post comment.')
    }
  }, 0)
}

const startEditComment = (comment: Comment) => {
  comment.isEditing = true
  comment.editContent = comment.content
  comment.editImage = null
  comment.editImagePreview = comment.imageUrl || null
}

const cancelEditComment = (comment: Comment) => {
  comment.isEditing = false
  comment.editContent = ''
  comment.editImage = null
  comment.editImagePreview = null
}

const saveEditComment = async (comment: Comment, post: Post) => {
  if (!comment.editContent) return

  try {
    const response = await fetch(`http://localhost:4000/api/comments/${comment.id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({
        commentContent: comment.editContent,
        commentImage: comment.editImage
      })
    })

    if (response.ok) {
      await loadPostDetails(post)
    } else {
      showErrorPopup('Error', 'Failed to update comment')
    }
  } catch (e) {
    showErrorPopup('Error', 'Network error')
  }
}

const deleteComment = async (commentId: string, post: Post) => {
  if (!(await showConfirmationPopup('Delete Comment', 'Are you sure?'))) return

  try {
    const res = await fetch(`http://localhost:4000/api/comments/${commentId}`, {
      method: 'DELETE',
      credentials: 'include'
    })
    if (res.ok) {
      post.displayComments = post.displayComments.filter((c) => c.id !== commentId)
      post.commentCount--
    }
  } catch (e) {
    console.error(e)
  }
}

const toggleLike = async (post: Post) => {
  const method = post.isReactedByActor ? 'DELETE' : 'POST'
  post.isReactedByActor = !post.isReactedByActor
  post.reactionCount += post.isReactedByActor ? 1 : -1

  try {
    await fetch(`http://localhost:4000/api/posts/${post.id}/reactions`, {
      method: method,
      credentials: 'include'
    })
  } catch (e) {
    post.isReactedByActor = !post.isReactedByActor
    post.reactionCount += post.isReactedByActor ? 1 : -1
  }
}

const removeCommentImage = (postId: string) => {
  if (commentInputs[postId]) {
    commentInputs[postId].image = null
    commentInputs[postId].imagePreview = null
  }
}

const removeCommentEditImage = (comment: Comment) => {
  comment.editImage = null
  comment.editImagePreview = null
}

const fetchActorUserId = async () => {
  const response = await fetch('http://localhost:4000/api/me', {
    method: 'GET',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include'
  })

  if (response.ok) {
    const body = await response.json()
    actorUserId.value = body.user_id
  }
}

const handleGlobalKeydown = (e: KeyboardEvent) => {
  if (!showCreateModal.value) return

  if (e.key === 'Escape') {
    showCreateModal.value = false
    return
  }

  if (e.key === 'Enter') {
    const target = e.target as HTMLElement
    if (target.tagName === 'TEXTAREA') {
      if (e.ctrlKey || e.metaKey) {
        handlePostSubmit()
      }
      return
    }

    e.preventDefault()
    handlePostSubmit()
  }
}

const togglePostMenu = (post: Post) => {
  closeAllMenus(post.id)
  post.isMenuOpen = !post.isMenuOpen
}

const toggleCommentMenu = (comment: Comment) => {
  closeAllMenus(undefined, comment.id)
  comment.isMenuOpen = !comment.isMenuOpen
}

const closeAllMenus = (exceptPostId?: string, exceptCommentId?: string) => {
  posts.value.forEach((p) => {
    if (p.id !== exceptPostId) p.isMenuOpen = false
    p.displayComments.forEach((c) => {
      if (c.id !== exceptCommentId) c.isMenuOpen = false
    })
  })
}

const handleGlobalClick = (_e: MouseEvent) => {
  closeAllMenus()
}

onMounted(async () => {
  window.addEventListener('keydown', handleGlobalKeydown)
  window.addEventListener('click', handleGlobalClick)
  if (!(await isLoggedIn())) {
    router.push('/signin')
    return
  }
  await fetchActorUserId()
  await fetchEventDetails()
  fetchPosts()
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleGlobalKeydown)
  window.removeEventListener('click', handleGlobalClick)
})
</script>

<template>
  <div class="min-h-screen bg-gray-50 text-gray-800 font-sans">
    <NavBar active="Events" />

    <main class="mx-auto max-w-[1400px] py-8 px-4">
      <div class="flex justify-between items-center mb-8">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">Discussion Channel</h1>
          <p class="text-gray-500 font-medium text-lg mt-1">{{ eventName }}</p>
        </div>
        <button
          @click="openCreateModal"
          class="bg-[#256EB1] text-white px-6 py-2 rounded-lg font-medium hover:bg-[#1d5a91] shadow-sm transition hover:cursor-pointer"
        >
          Create New Post
        </button>
      </div>
      <div class="mb-6">
        <button
          @click="router.back()"
          class="hover:cursor-pointer bg-gray-200 hover:bg-gray-300 text-gray-800 transition rounded-md p-2"
        >
          ← Back
        </button>
      </div>
      <div v-if="isLoading" class="flex justify-center py-12">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-[#256EB1]"></div>
      </div>

      <div v-else-if="posts.length === 0" class="text-center py-12 bg-white rounded-xl shadow-sm">
        <p class="text-gray-500 text-lg">No discussions yet. Be the first to post!</p>
      </div>

      <div v-else class="space-y-6">
        <div
          v-for="post in posts"
          :key="post.id"
          class="bg-white rounded-xl shadow-sm overflow-hidden flex flex-col md:flex-row min-h-fit"
        >
          <div class="w-full md:w-[60%] p-6 border-b md:border-b-0 md:border-r border-gray-100 flex flex-col">
            <div class="flex justify-between items-start mb-4">
              <div class="flex items-center gap-3">
                <img
                  v-if="post.author?.avatarUrl"
                  :src="post.author.avatarUrl"
                  class="h-10 w-10 rounded-full object-cover border border-gray-200"
                />
                <div
                  v-else
                  class="h-10 w-10 rounded-full bg-gray-200 flex items-center justify-center text-gray-500 font-bold"
                >
                  {{ (post.author?.fullName || post.author?.username || '?').charAt(0).toUpperCase() }}
                </div>

                <div>
                  <h3 class="font-semibold text-gray-900">
                    {{ post.author?.fullName || post.author?.username || 'Unknown User' }}
                  </h3>
                  <span class="text-xs text-gray-500">{{ formatDate(post.lastUpdatedAt) }}</span>
                </div>
              </div>

              <div v-if="isOwner(post.author)" class="relative">
                <button
                  @click.stop="togglePostMenu(post)"
                  class="text-gray-400 hover:text-gray-600 p-1 hover:cursor-pointer rounded-full hover:bg-gray-100 transition"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-5 w-5"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"
                    />
                  </svg>
                </button>

                <div
                  v-if="post.isMenuOpen"
                  class="absolute right-0 mt-1 w-32 bg-white border shadow-lg rounded-md z-10"
                  @click.stop
                >
                  <button
                    @click="
                      openEditPostModal(post);
                      post.isMenuOpen = false
                    "
                    class="block rounded-t-md w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 hover:cursor-pointer"
                  >
                    Edit Post
                  </button>
                  <button
                    @click="
                      deletePost(post.id);
                      post.isMenuOpen = false
                    "
                    class="block rounded-b-md w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-gray-100 hover:cursor-pointer"
                  >
                    Delete Post
                  </button>
                </div>
              </div>
            </div>

            <div class="flex-1">
              <h2 class="text-xl font-bold mb-2">{{ post.title }}</h2>
              <p class="text-gray-800 mb-4 whitespace-pre-wrap">{{ post.content }}</p>
              <div
                v-if="post.imageUrl"
                class="mb-4 rounded-lg overflow-hidden max-h-80 flex items-center justify-center"
              >
                <img :src="post.imageUrl" alt="Post Image" class="max-w-full max-h-full object-contain" />
              </div>
            </div>
          </div>

          <div class="w-full md:w-[40%] bg-gray-50 flex flex-col h-full">
            <h4
              class="pt-4 pl-4 pb-2 border-b border-gray-300 text-sm font-bold text-gray-500 uppercase tracking-wider"
            >
              Comments
            </h4>
            <div class="flex-1 p-4 overflow-y-auto max-h-[500px]">
              <div
                v-if="!post.areCommentsLoaded && post.commentCount > post.displayComments.length"
                class="text-center mb-4"
              >
                <button
                  @click="loadPostDetails(post)"
                  class="text-[#256EB1] text-sm hover:underline hover:cursor-pointer"
                >
                  View all {{ post.commentCount }} comments
                </button>
              </div>

              <div v-if="post.displayComments.length === 0" class="text-center text-gray-400 text-sm italic py-4">
                No comments yet.
              </div>

              <div v-for="comment in post.displayComments" :key="comment.id" class="mb-4 group">
                <div class="flex gap-3">
                  <img
                    v-if="comment.author?.avatarUrl"
                    :src="comment.author.avatarUrl"
                    class="h-8 w-8 rounded-full object-cover shrink-0 border border-gray-200"
                  />
                  <div
                    v-else
                    class="h-8 w-8 rounded-full bg-gray-300 shrink-0 flex items-center justify-center text-xs font-bold text-white"
                  >
                    {{ (comment.author?.fullName || comment.author?.username || 'U').charAt(0).toUpperCase() }}
                  </div>

                  <div class="flex-1">
                    <div v-if="comment.isEditing" class="bg-white p-3 rounded-2xl shadow-sm border border-blue-200">
                      <textarea
                        v-model="comment.editContent"
                        class="w-full text-sm border-gray-200 rounded p-1 mb-2 outline-none focus:border-blue-300 border resize-none"
                        rows="8"
                      ></textarea>

                      <div v-if="comment.editImagePreview" class="relative mb-2 inline-block">
                        <img :src="comment.editImagePreview" class="h-16 w-auto object-cover rounded" />
                        <button
                          @click="removeCommentEditImage(comment)"
                          class="absolute -top-1 -right-1 bg-red-500 text-white rounded-full p-0.5 hover:cursor-pointer"
                        >
                          <svg class="h-3 w-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M6 18L18 6M6 6l12 12"
                            />
                          </svg>
                        </button>
                      </div>

                      <div class="flex justify-between items-center mt-1">
                        <label class="cursor-pointer text-gray-400 hover:text-gray-600 hover:cursor-pointer">
                          <input
                            type="file"
                            class="hidden"
                            accept="image/*"
                            @change="(e) => handleFileUpload(e, 'comment-edit', undefined, comment)"
                          />
                          <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                          >
                            <path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="2"
                              d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                            />
                          </svg>
                        </label>
                        <div class="flex gap-2">
                          <button
                            @click="cancelEditComment(comment)"
                            class="text-xs text-gray-500 hover:text-gray-700 hover:cursor-pointer"
                          >
                            Cancel
                          </button>
                          <button
                            @click="saveEditComment(comment, post)"
                            class="text-xs bg-[#256EB1] text-white px-3 py-1 rounded hover:bg-[#1d5a91] hover:cursor-pointer"
                          >
                            Save
                          </button>
                        </div>
                      </div>
                    </div>

                    <div v-else>
                      <div class="bg-white p-3 rounded-2xl rounded-tl-none shadow-sm border border-gray-100 relative">
                        <div class="flex justify-between items-start mb-1">
                          <span class="text-sm font-bold text-gray-900">
                            {{ comment.author?.fullName || comment.author?.username || 'Unknown' }}
                          </span>

                          <div v-if="isOwner(comment.author)" class="relative">
                            <button
                              @click.stop="toggleCommentMenu(comment)"
                              class="text-gray-400 hover:text-gray-500 hover:cursor-pointer rounded-full p-0.5 hover:bg-gray-100 transition"
                            >
                              <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-4 w-4"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                              >
                                <path
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                  stroke-width="2"
                                  d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"
                                />
                              </svg>
                            </button>

                            <div
                              v-if="comment.isMenuOpen"
                              class="absolute right-0 top-6 bg-white border shadow-md rounded z-20 w-20"
                              @click.stop
                            >
                              <button
                                @click="
                                  startEditComment(comment);
                                  comment.isMenuOpen = false
                                "
                                class="text-xs text-gray-700 px-2 py-1 hover:bg-gray-100 w-full text-left hover:cursor-pointer rounded-t-md"
                              >
                                Edit
                              </button>
                              <button
                                @click="
                                  deleteComment(comment.id, post);
                                  comment.isMenuOpen = false
                                "
                                class="text-xs text-red-600 px-2 py-1 hover:bg-gray-100 w-full text-left hover:cursor-pointer rounded-b-md"
                              >
                                Delete
                              </button>
                            </div>
                          </div>
                        </div>
                        <p class="text-sm text-gray-700">{{ comment.content }}</p>
                        <img
                          v-if="comment.imageUrl"
                          :src="comment.imageUrl"
                          class="mt-2 rounded-lg w-full h-32 object-cover"
                        />
                      </div>
                      <span class="text-[10px] text-gray-400 ml-1">{{ formatDate(comment.lastUpdatedAt) }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div class="p-4 bg-white border-t border-gray-200">
              <div class="flex gap-4 mb-3 text-sm text-gray-600">
                <button
                  @click="toggleLike(post)"
                  class="flex items-center gap-1 hover:bg-gray-50 px-2 py-1 rounded transition hover:cursor-pointer"
                  :class="post.isReactedByActor ? 'text-blue-600 font-bold' : ''"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-5 w-5"
                    :fill="post.isReactedByActor ? 'currentColor' : 'none'"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M14 10h4.764a2 2 0 011.789 2.894l-3.5 7A2 2 0 0115.263 21h-4.017c-.163 0-.326-.02-.485-.06L7 20m7-10V5a2 2 0 00-2-2h-.095c-.5 0-.905.405-.905.905 0 .714-.211 1.412-.608 2.006L7 11v9m7-10h-2M7 20H5a2 2 0 01-2-2v-6a2 2 0 012-2h2.5"
                    />
                  </svg>
                  {{ post.reactionCount }}
                </button>
                <div class="flex items-center gap-1 px-2 py-1">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-5 w-5"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"
                    />
                  </svg>
                  {{ post.commentCount }}
                </div>
              </div>

              <div v-if="commentInputs[post.id]" class="bg-gray-100 rounded-[20px] px-4 py-2">
                <div v-if="commentInputs[post.id]!.imagePreview" class="relative mb-2 inline-block">
                  <div class="relative rounded-lg overflow-hidden border border-gray-200">
                    <img :src="commentInputs[post.id]!.imagePreview!" class="h-20 w-auto object-cover" />
                    <button
                      @click="removeCommentImage(post.id)"
                      class="absolute top-1 right-1 bg-gray-800/70 hover:bg-gray-900 text-white rounded-full p-0.5 hover:cursor-pointer"
                    >
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-4 w-4"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          stroke-width="2"
                          d="M6 18L18 6M6 6l12 12"
                        />
                      </svg>
                    </button>
                  </div>
                </div>

                <div class="flex items-center gap-2">
                  <input
                    v-model="commentInputs[post.id]!.text"
                    @keyup.enter="postComment(post)"
                    type="text"
                    placeholder="Write a comment..."
                    class="bg-transparent border-none focus:ring-0 flex-1 text-sm outline-none placeholder-gray-500 text-gray-900"
                  />

                  <label
                    class="cursor-pointer text-gray-400 hover:text-gray-600 hover:cursor-pointer flex items-center"
                  >
                    <input
                      type="file"
                      class="hidden"
                      accept="image/*"
                      @change="(e) => handleFileUpload(e, 'comment', post.id)"
                    />
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      class="h-5 w-5"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                      />
                    </svg>
                  </label>

                  <button
                    @click="postComment(post)"
                    class="text-[#256EB1] hover:text-[#1d5a91] hover:cursor-pointer flex items-center"
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      class="h-5 w-5 transform rotate-90"
                      fill="currentColor"
                      viewBox="0 0 20 20"
                    >
                      <path
                        d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z"
                      />
                    </svg>
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <div v-if="showCreateModal" class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4">
      <div class="bg-white rounded-xl shadow-2xl w-full max-w-lg overflow-hidden">
        <div class="p-6 border-b border-gray-100 flex justify-between items-center">
          <h3 class="text-xl font-bold text-gray-900">{{ editingPostId ? 'Edit Post' : 'Create Post' }}</h3>
          <button
            @click="showCreateModal = false"
            class="hover:cursor-pointer hover:bg-gray-100 transition rounded-full p-1"
          >
            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        <div class="p-6 space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Title</label>
            <input
              v-model="newPost.title"
              type="text"
              class="transition w-full border border-gray-300 rounded-lg px-3 py-2 outline-none focus:ring-2 focus:ring-[#256EB1]"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Content</label>
            <textarea
              v-model="newPost.content"
              rows="4"
              class="transition w-full h-60 resize-none border border-gray-300 rounded-lg px-3 py-2 outline-none focus:ring-2 focus:ring-[#256EB1]"
            ></textarea>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Image (Optional)</label>
            <div class="border-2 border-dashed border-gray-300 rounded-lg p-4 text-center hover:bg-gray-50 relative">
              <input
                type="file"
                @change="(e) => handleFileUpload(e, 'post')"
                class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
                accept="image/*"
              />
              <div v-if="!newPost.imagePreview" class="text-gray-500">Click to upload</div>
              <div v-else class="relative">
                <img :src="newPost.imagePreview" class="h-32 mx-auto object-contain" />
                <button
                  @click.prevent.stop="
                    newPost.image = null;
                    newPost.imagePreview = null
                  "
                  class="absolute top-0 right-0 bg-red-500 text-white rounded-full p-1 hover:cursor-pointer"
                >
                  <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </div>
        <div class="p-6 bg-gray-50 flex justify-end gap-3">
          <button
            @click="showCreateModal = false"
            class="px-4 py-2 text-gray-800 bg-gray-200 hover:bg-gray-300 hover:scale-[103%] rounded-lg hover:cursor-pointer transition"
          >
            Cancel
          </button>
          <button
            @click="handlePostSubmit"
            :disabled="isSubmitting"
            class="px-6 py-2 bg-[#256EB1] text-white rounded-lg hover:bg-[#1d5a91] disabled:opacity-50 hover:cursor-pointer transition hover:scale-[103%]"
          >
            {{ isSubmitting ? (editingPostId ? 'Updating...' : 'Posting...') : editingPostId ? 'Update' : 'Post' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
