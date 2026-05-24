<script lang="ts">
  import * as api from '$lib/api';
  import type { User, Blog } from '$lib/api';
  import Button   from '$lib/components/ui/button.svelte';
  import Input    from '$lib/components/ui/input.svelte';
  import Textarea from '$lib/components/ui/textarea.svelte';
  import Badge    from '$lib/components/ui/badge.svelte';
  import Label    from '$lib/components/ui/label.svelte';
  import { cn }   from '$lib/utils';

  // ── Tab ──────────────────────────────────────────────────
  type Tab = 'users' | 'blogs';
  let activeTab = $state<Tab>('users');

  // ── Create User ──────────────────────────────────────────
  let cu = $state({
    username: '',
    name:     '',
    loading:  false,
    result:   null as User | null,
    error:    null as string | null
  });

  async function handleCreateUser() {
    cu.error  = null;
    cu.result = null;
    cu.loading = true;
    try {
      cu.result = await api.createUser({ username: cu.username.trim(), name: cu.name.trim() });
    } catch (e) {
      cu.error = (e as Error).message;
    } finally {
      cu.loading = false;
    }
  }

  // ── Get User ─────────────────────────────────────────────
  let gu = $state({
    username: '',
    loading:  false,
    result:   null as User | null,
    error:    null as string | null
  });

  async function handleGetUser() {
    gu.error  = null;
    gu.result = null;
    gu.loading = true;
    try {
      gu.result = await api.getUser(gu.username.trim());
    } catch (e) {
      gu.error = (e as Error).message;
    } finally {
      gu.loading = false;
    }
  }

  // ── Create Blog ──────────────────────────────────────────
  let cb = $state({
    username: '',
    name:     '',
    tweet:    '',
    loading:  false,
    result:   null as Blog | null,
    error:    null as string | null
  });

  async function handleCreateBlog() {
    cb.error  = null;
    cb.result = null;
    cb.loading = true;
    try {
      cb.result = await api.createBlog({
        username: cb.username.trim(),
        name:     cb.name.trim(),
        tweet:    cb.tweet.trim()
      });
    } catch (e) {
      cb.error = (e as Error).message;
    } finally {
      cb.loading = false;
    }
  }

  // ── Get All Blogs ─────────────────────────────────────────
  let gdb = $state({
    loading: false,
    result:  null as Blog[] | null,
    error:   null as string | null
  });

  async function handleGetBlogs() {
    gdb.error  = null;
    gdb.result = null;
    gdb.loading = true;
    try {
      gdb.result = await api.getBlogs();
    } catch (e) {
      gdb.error = (e as Error).message;
    } finally {
      gdb.loading = false;
    }
  }

  // ── Helpers ───────────────────────────────────────────────
  const fmt = (v: unknown) => JSON.stringify(v, null, 2);

  function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text).catch(() => {});
  }
</script>

<!-- ═══════════════════════════════════════════════════════════ -->
<div class="min-h-screen bg-background flex flex-col">

  <!-- ── Topbar ── -->
  <header class="sticky top-0 z-20 border-b border-border bg-card/80 backdrop-blur-md">
    <div class="max-w-6xl mx-auto px-6 h-14 flex items-center justify-between">

      <div class="flex items-center gap-3">
        <span class="text-xl leading-none">🦀</span>
        <div class="leading-tight">
          <p class="text-sm font-semibold text-foreground">Rust API Explorer</p>
          <p class="text-[11px] text-muted-foreground font-mono">Axum · Tokio · Serde</p>
        </div>
      </div>

      <!-- Backend status pill -->
      <a
        href="http://127.0.0.1:3000"
        target="_blank"
        rel="noopener noreferrer"
        class="flex items-center gap-2 rounded-full border border-border bg-muted/50
               px-3 py-1.5 text-xs text-muted-foreground font-mono
               hover:text-foreground transition-colors"
      >
        <span class="relative flex h-2 w-2">
          <span class="absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75 animate-ping"></span>
          <span class="relative inline-flex h-2 w-2 rounded-full bg-green-500"></span>
        </span>
        127.0.0.1:3000
      </a>
    </div>
  </header>

  <!-- ── Hero ── -->
  <div class="border-b border-border bg-gradient-to-b from-card/60 to-background">
    <div class="max-w-6xl mx-auto px-6 py-10">
      <p class="text-xs font-semibold text-primary uppercase tracking-widest mb-2">REST API</p>
      <h1 class="text-3xl font-bold tracking-tight text-foreground mb-2">API Explorer</h1>
      <p class="text-muted-foreground max-w-xl text-sm leading-relaxed">
        Send live requests to the Rust/Axum backend running at
        <code class="rounded bg-muted px-1.5 py-0.5 text-[12px] font-mono text-foreground">
          http://127.0.0.1:3000
        </code>.
        Responses include RFC 3339 timestamps generated server-side.
      </p>

      <!-- Endpoint summary pills -->
      <div class="flex flex-wrap gap-2 mt-5">
        {#each [
          { method: 'POST', path: '/user' },
          { method: 'GET',  path: '/user' },
          { method: 'POST', path: '/blog' },
          { method: 'GET',  path: '/blog' }
        ] as ep}
          <span class="flex items-center gap-1.5 rounded-full border border-border bg-muted/40 px-3 py-1 text-xs font-mono">
            <Badge variant={ep.method === 'POST' ? 'post' : 'get'}>{ep.method}</Badge>
            <span class="text-foreground">{ep.path}</span>
          </span>
        {/each}
      </div>
    </div>
  </div>

  <!-- ── Main ── -->
  <main class="flex-1 max-w-6xl mx-auto w-full px-6 py-8">

    <!-- Tab switcher -->
    <div class="flex gap-1 w-fit rounded-xl border border-border bg-muted/30 p-1 mb-8">
      {#each [
        { id: 'users' as Tab, emoji: '👤', label: 'Users' },
        { id: 'blogs' as Tab, emoji: '📝', label: 'Blogs'  }
      ] as tab}
        <button
          class={cn(
            'flex items-center gap-2 px-5 py-2 rounded-lg text-sm font-medium transition-all duration-150',
            activeTab === tab.id
              ? 'bg-background text-foreground shadow-sm border border-border'
              : 'text-muted-foreground hover:text-foreground'
          )}
          onclick={() => (activeTab = tab.id)}
        >
          <span class="text-base">{tab.emoji}</span>
          {tab.label}
        </button>
      {/each}
    </div>

    <!-- ══ USERS TAB ══════════════════════════════════════════ -->
    {#if activeTab === 'users'}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">

      <!-- ── Card: POST /user ── -->
      <div class="rounded-xl border border-border bg-card flex flex-col overflow-hidden">
        <!-- Card header -->
        <div class="px-5 py-4 border-b border-border bg-muted/20">
          <div class="flex items-center gap-2 mb-0.5">
            <Badge variant="post">POST</Badge>
            <code class="text-sm font-mono text-foreground">/user</code>
          </div>
          <p class="text-xs text-muted-foreground">Create a new user — returns 201 with server-assigned timestamp</p>
        </div>

        <!-- Form -->
        <div class="p-5 space-y-4 flex-1">
          <div class="space-y-1.5">
            <Label for="cu-username">Username</Label>
            <Input
              id="cu-username"
              placeholder="e.g. alice"
              bind:value={cu.username}
              onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && !cu.loading && cu.username && cu.name && handleCreateUser()}
            />
          </div>
          <div class="space-y-1.5">
            <Label for="cu-name">Full Name</Label>
            <Input
              id="cu-name"
              placeholder="e.g. Alice Smith"
              bind:value={cu.name}
              onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && !cu.loading && cu.username && cu.name && handleCreateUser()}
            />
          </div>

          <!-- Request preview -->
          <div class="rounded-md border border-border bg-muted/30 px-3 py-2">
            <p class="text-[10px] text-muted-foreground font-mono uppercase tracking-wider mb-1">Request body</p>
            <pre class="text-xs text-foreground/80 font-mono">{fmt({ username: cu.username || '<username>', name: cu.name || '<name>' })}</pre>
          </div>

          <Button
            class="w-full"
            loading={cu.loading}
            disabled={!cu.username.trim() || !cu.name.trim()}
            onclick={handleCreateUser}
          >
            Create User
          </Button>
        </div>

        <!-- Response panel -->
        {#if cu.result || cu.error}
        <div class="border-t border-border px-5 py-4 bg-muted/10">
          <div class="flex items-center justify-between mb-2">
            <p class="text-[10px] font-semibold uppercase tracking-widest text-muted-foreground">Response</p>
            {#if cu.result}
              <div class="flex items-center gap-2">
                <Badge variant="success">201 Created</Badge>
                <button
                  class="text-[10px] text-muted-foreground hover:text-foreground transition-colors font-mono"
                  onclick={() => cu.result && copyToClipboard(fmt(cu.result))}
                >copy</button>
              </div>
            {:else}
              <Badge variant="error">Error</Badge>
            {/if}
          </div>

          {#if cu.error}
            <p class="text-xs text-destructive font-mono bg-destructive/10 border border-destructive/20 rounded-md px-3 py-2">
              {cu.error}
            </p>
          {:else if cu.result}
            <pre class="text-xs text-foreground font-mono bg-muted/40 border border-border rounded-md px-3 py-2 overflow-auto">{fmt(cu.result)}</pre>
          {/if}
        </div>
        {/if}
      </div>

      <!-- ── Card: GET /user ── -->
      <div class="rounded-xl border border-border bg-card flex flex-col overflow-hidden">
        <div class="px-5 py-4 border-b border-border bg-muted/20">
          <div class="flex items-center gap-2 mb-0.5">
            <Badge variant="get">GET</Badge>
            <code class="text-sm font-mono text-foreground">/user</code>
          </div>
          <p class="text-xs text-muted-foreground">Retrieve a user by username query parameter</p>
        </div>

        <div class="p-5 space-y-4 flex-1">
          <div class="space-y-1.5">
            <Label for="gu-username">Username</Label>
            <div class="flex gap-2">
              <Input
                id="gu-username"
                placeholder="e.g. alice"
                bind:value={gu.username}
                onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && !gu.loading && gu.username && handleGetUser()}
              />
              <Button
                variant="outline"
                loading={gu.loading}
                disabled={!gu.username.trim()}
                onclick={handleGetUser}
              >
                Fetch
              </Button>
            </div>
          </div>

          <!-- URL preview -->
          <div class="rounded-md border border-border bg-muted/30 px-3 py-2">
            <p class="text-[10px] text-muted-foreground font-mono uppercase tracking-wider mb-1">Request URL</p>
            <p class="text-xs font-mono text-foreground/80 break-all">
              GET /user?username=<span class="text-primary">{gu.username || '&lt;username&gt;'}</span>
            </p>
          </div>
        </div>

        {#if gu.result || gu.error}
        <div class="border-t border-border px-5 py-4 bg-muted/10">
          <div class="flex items-center justify-between mb-2">
            <p class="text-[10px] font-semibold uppercase tracking-widest text-muted-foreground">Response</p>
            {#if gu.result}
              <div class="flex items-center gap-2">
                <Badge variant="success">200 OK</Badge>
                <button
                  class="text-[10px] text-muted-foreground hover:text-foreground transition-colors font-mono"
                  onclick={() => gu.result && copyToClipboard(fmt(gu.result))}
                >copy</button>
              </div>
            {:else}
              <Badge variant="error">Error</Badge>
            {/if}
          </div>
          {#if gu.error}
            <p class="text-xs text-destructive font-mono bg-destructive/10 border border-destructive/20 rounded-md px-3 py-2">
              {gu.error}
            </p>
          {:else if gu.result}
            <pre class="text-xs text-foreground font-mono bg-muted/40 border border-border rounded-md px-3 py-2 overflow-auto">{fmt(gu.result)}</pre>
          {/if}
        </div>
        {/if}
      </div>

    </div>
    {/if}

    <!-- ══ BLOGS TAB ══════════════════════════════════════════ -->
    {#if activeTab === 'blogs'}
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">

      <!-- ── Card: POST /blog ── -->
      <div class="rounded-xl border border-border bg-card flex flex-col overflow-hidden">
        <div class="px-5 py-4 border-b border-border bg-muted/20">
          <div class="flex items-center gap-2 mb-0.5">
            <Badge variant="post">POST</Badge>
            <code class="text-sm font-mono text-foreground">/blog</code>
          </div>
          <p class="text-xs text-muted-foreground">Create a blog post — returns 201 with embedded user and timestamp</p>
        </div>

        <div class="p-5 space-y-4 flex-1">
          <div class="space-y-1.5">
            <Label for="cb-username">Username</Label>
            <Input id="cb-username" placeholder="e.g. alice" bind:value={cb.username} />
          </div>
          <div class="space-y-1.5">
            <Label for="cb-name">Full Name</Label>
            <Input id="cb-name" placeholder="e.g. Alice Smith" bind:value={cb.name} />
          </div>
          <div class="space-y-1.5">
            <Label for="cb-tweet">Post Content</Label>
            <Textarea
              id="cb-tweet"
              placeholder="What's on your mind?"
              rows={3}
              bind:value={cb.tweet}
            />
            <p class="text-[11px] text-muted-foreground text-right font-mono">
              {cb.tweet.length} chars
            </p>
          </div>

          <Button
            class="w-full"
            loading={cb.loading}
            disabled={!cb.username.trim() || !cb.name.trim() || !cb.tweet.trim()}
            onclick={handleCreateBlog}
          >
            Publish Post
          </Button>
        </div>

        {#if cb.result || cb.error}
        <div class="border-t border-border px-5 py-4 bg-muted/10">
          <div class="flex items-center justify-between mb-2">
            <p class="text-[10px] font-semibold uppercase tracking-widest text-muted-foreground">Response</p>
            {#if cb.result}
              <div class="flex items-center gap-2">
                <Badge variant="success">201 Created</Badge>
                <button
                  class="text-[10px] text-muted-foreground hover:text-foreground transition-colors font-mono"
                  onclick={() => cb.result && copyToClipboard(fmt(cb.result))}
                >copy</button>
              </div>
            {:else}
              <Badge variant="error">Error</Badge>
            {/if}
          </div>
          {#if cb.error}
            <p class="text-xs text-destructive font-mono bg-destructive/10 border border-destructive/20 rounded-md px-3 py-2">
              {cb.error}
            </p>
          {:else if cb.result}
            <pre class="text-xs text-foreground font-mono bg-muted/40 border border-border rounded-md px-3 py-2 overflow-auto">{fmt(cb.result)}</pre>
          {/if}
        </div>
        {/if}
      </div>

      <!-- ── Card: GET /blog ── -->
      <div class="rounded-xl border border-border bg-card flex flex-col overflow-hidden">
        <div class="px-5 py-4 border-b border-border bg-muted/20">
          <div class="flex items-center gap-2 mb-0.5">
            <Badge variant="get">GET</Badge>
            <code class="text-sm font-mono text-foreground">/blog</code>
          </div>
          <p class="text-xs text-muted-foreground">Returns all blog posts from SQLite, newest first</p>
        </div>

        <div class="p-5 space-y-4 flex-1">
          <p class="text-sm text-muted-foreground leading-relaxed">
            Fetches every blog post stored in SQLite, ordered by newest first.
            Each post contains a nested <code class="rounded bg-muted px-1 text-xs font-mono text-foreground">user</code>
            object and an RFC&nbsp;3339 <code class="rounded bg-muted px-1 text-xs font-mono text-foreground">timestamp</code>.
          </p>

          <!-- URL preview -->
          <div class="rounded-md border border-border bg-muted/30 px-3 py-2">
            <p class="text-[10px] text-muted-foreground font-mono uppercase tracking-wider mb-1">Request URL</p>
            <p class="text-xs font-mono text-foreground/80">GET /blog</p>
          </div>

          <Button
            variant="outline"
            class="w-full"
            loading={gdb.loading}
            onclick={handleGetBlogs}
          >
            Fetch All Posts
          </Button>
        </div>

        {#if gdb.result !== null || gdb.error}
        <div class="border-t border-border px-5 py-4 bg-muted/10">
          <div class="flex items-center justify-between mb-3">
            <p class="text-[10px] font-semibold uppercase tracking-widest text-muted-foreground">Response</p>
            {#if gdb.result !== null}
              <div class="flex items-center gap-2">
                <Badge variant="success">200 OK</Badge>
                <span class="text-xs text-muted-foreground font-mono">
                  {gdb.result.length} post{gdb.result.length !== 1 ? 's' : ''}
                </span>
              </div>
            {:else}
              <Badge variant="error">Error</Badge>
            {/if}
          </div>

          {#if gdb.error}
            <p class="text-xs text-destructive font-mono bg-destructive/10 border border-destructive/20 rounded-md px-3 py-2">
              {gdb.error}
            </p>
          {:else if gdb.result !== null}
            {#if gdb.result.length === 0}
              <div class="rounded-md border border-dashed border-border bg-muted/10 px-4 py-6 text-center">
                <p class="text-sm text-muted-foreground">No blog posts yet.</p>
                <p class="text-xs text-muted-foreground mt-1">Create one using the form on the left!</p>
              </div>
            {:else}
              <div class="space-y-2 max-h-72 overflow-y-auto pr-1">
                {#each gdb.result as post}
                  <div class="rounded-md border border-border bg-muted/30 px-3 py-2.5">
                    <div class="flex items-start justify-between gap-2 mb-1.5">
                      <span class="text-xs font-semibold text-foreground font-mono">@{post.user.username}</span>
                      <span class="text-[10px] text-muted-foreground font-mono shrink-0">
                        {new Date(post.timestamp).toLocaleString()}
                      </span>
                    </div>
                    <p class="text-sm text-foreground/90 leading-relaxed">{post.tweet}</p>
                  </div>
                {/each}
              </div>
            {/if}
          {/if}
        </div>
        {/if}
      </div>

    </div>
    {/if}

  </main>

  <!-- ── Footer ── -->
  <footer class="border-t border-border mt-auto">
    <div class="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
      <p class="text-xs text-muted-foreground font-mono">
        🦀 Axum 0.8 · Tokio 1 · Serde · time 0.3
      </p>
      <p class="text-xs text-muted-foreground font-mono">
        Svelte 5 · Tailwind · shadcn-svelte
      </p>
    </div>
  </footer>

</div>
