<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  interface Props extends HTMLButtonAttributes {
    variant?: 'default' | 'outline' | 'ghost' | 'destructive';
    size?: 'default' | 'sm' | 'lg' | 'icon';
    loading?: boolean;
    children?: Snippet;
  }

  let {
    class: className = '',
    variant = 'default',
    size = 'default',
    loading = false,
    disabled,
    children,
    ...rest
  }: Props = $props();

  const variants: Record<string, string> = {
    default:     'bg-primary text-primary-foreground shadow hover:bg-primary/90',
    outline:     'border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground',
    ghost:       'hover:bg-accent hover:text-accent-foreground',
    destructive: 'bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90'
  };

  const sizes: Record<string, string> = {
    default: 'h-9 px-4 py-2',
    sm:      'h-8 rounded-md px-3 text-xs',
    lg:      'h-10 rounded-md px-8',
    icon:    'h-9 w-9'
  };
</script>

<button
  class={cn(
    'inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors',
    'focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring',
    'disabled:pointer-events-none disabled:opacity-50',
    variants[variant],
    sizes[size],
    className
  )}
  disabled={disabled || loading}
  {...rest}
>
  {#if loading}
    <svg
      class="h-4 w-4 animate-spin"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962
           7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>
  {/if}
  {@render children?.()}
</button>
