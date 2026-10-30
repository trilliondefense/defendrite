<script lang="ts">
  type CaseStatus = 'Active' | 'Review' | 'Closed';
   import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";

  interface Conversation {
    id: string;
    title: string;
    chatNumber: number;
    createdAt: number;
    updatedAt: number;
  }

  let chats = $state<Conversation[]>([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  onMount(() => {
    loadChats();
  });

  async function loadChats() {
    isLoading = true;
    error = null;

    try {
      chats = await invoke<Conversation[]>("list_chats");
    } catch (err) {
      console.error("Failed to load chats:", err);
      error = "Could not load conversation history.";
    } finally {
      isLoading = false;
    }
  }

  async function openChat(chatId: string) {
    await goto(`/investigations/${chatId}`);
  }

  function formatDate(timestamp: number) {
    return new Intl.DateTimeFormat("en", {
      dateStyle: "medium",
      timeStyle: "short"
    }).format(timestamp * 1000);
  }

  type ForensicCase = {
    id: string;
    title: string;
    reference: string;
    type: string;
    investigator: string;
    status: CaseStatus;
    evidence: number;
    createdAt: string;
  };

  let cases = $state<ForensicCase[]>([
    {
      id: '1',
      title: 'Corporate Data Exfiltration',
      reference: 'DF-2026-001',
      type: 'Incident Response',
      investigator: 'A. Mitchell',
      status: 'Active',
      evidence: 24,
      createdAt: '2026-10-18'
    },
    {
      id: '2',
      title: 'Mobile Device Examination',
      reference: 'DF-2026-002',
      type: 'Mobile Forensics',
      investigator: 'J. Parker',
      status: 'Review',
      evidence: 12,
      createdAt: '2026-10-16'
    },
    {
      id: '3',
      title: 'Unauthorized System Access',
      reference: 'DF-2026-003',
      type: 'Computer Forensics',
      investigator: 'S. Morgan',
      status: 'Active',
      evidence: 8,
      createdAt: '2026-10-14'
    }
  ]);

  let search = $state('');
  let statusFilter = $state('All');
  let showCreateForm = $state(false);

  let title = $state('');
  let type = $state('Computer Forensics');
  let investigator = $state('');

  let filteredCases = $derived(
    cases.filter((item) => {
      const query = search.toLowerCase();

      const matchesSearch =
        `${item.title} ${item.reference} ${item.investigator} ${item.type}`
          .toLowerCase()
          .includes(query);

      const matchesStatus =
        statusFilter === 'All' || item.status === statusFilter;

      return matchesSearch && matchesStatus;
    })
  );

  let activeCases = $derived(
    cases.filter((item) => item.status === 'Active').length
  );

  let reviewCases = $derived(
    cases.filter((item) => item.status === 'Review').length
  );

  let evidenceCount = $derived(
    cases.reduce((total, item) => total + item.evidence, 0)
  );

  function createCase() {
    if (!title.trim() || !investigator.trim()) return;

    const number =
      Math.max(
        0,
        ...cases.map((item) => {
          const match = item.reference.match(/DF-\d+-(\d+)/);
          return match ? Number(match[1]) : 0;
        })
      ) + 1;

    cases = [
      {
        id: crypto.randomUUID(),
        title: title.trim(),
        reference: `DF-2026-${String(number).padStart(3, '0')}`,
        type,
        investigator: investigator.trim(),
        status: 'Active',
        evidence: 0,
        createdAt: new Date().toISOString().slice(0, 10)
      },
      ...cases
    ];

    title = '';
    investigator = '';
    showCreateForm = false;
  }
</script>

<svelte:head>
  <title>Investigations | Forensic Workspace</title>
</svelte:head>

<div class="investigations-page">
  <header class="page-header">
    <div class="eyebrow">FORENSIC WORKSPACE</div>
    <h1>Investigations</h1>
    <p>
      Manage forensic cases, track examination progress, and organize
      digital evidence in one workspace.
    </p>
  </header>

  <div class="toolbar">
    <label class="search-box">
      <span>⌕</span>
      <input
        type="search"
        placeholder="Search investigations..."
        aria-label="Search investigations"
      />
    </label>

    <button class="primary-button">+ New investigation</button>
  </div>

  <section class="overview">
    <div class="stat-card">
      <span>Total investigations</span>
      <strong>32</strong>
      <small>Across your workspace</small>
    </div>

    <div class="stat-card">
      <span>Active</span>
      <strong>12</strong>
      <small>Currently under examination</small>
    </div>

    <div class="stat-card">
      <span>Completed</span>
      <strong>20</strong>
      <small>Investigations closed</small>
    </div>
  </section>

  <!-- <section class="investigation-section">
    <div class="section-header">
      <div>
        <h2>All investigations</h2>
        <p>Review cases and their current progress</p>
      </div>

      <button class="filter-button">All cases⌄</button>
    </div>

    <div class="investigation-list">
      <div class="investigation-row table-header">
        <span>Investigation</span>
        <span>Case ID</span>
        <span>Priority</span>
        <span>Status</span>
        <span>Updated</span>
        <span></span>
      </div>

      <div class="investigation-row">
        <div class="case-name">
          <div class="case-icon">01</div>
          <div>
            <strong>Mobile Device Examination</strong>
            <small>Device acquisition and artifact analysis</small>
          </div>
        </div>
        <span class="case-id">CASE-024</span>
        <span class="priority high">High</span>
        <span class="status active">In progress</span>
        <span class="date">Oct 24, 2026</span>
        <button class="more-button" aria-label="Case options">···</button>
      </div>

    </div>

    <footer class="list-footer">
      <span>Showing 5 of 32 investigations</span>
      <div class="pagination">
        <button disabled>Previous</button>
        <button class="current-page">1</button>
        <button>2</button>
        <button>3</button>
        <button>Next</button>
      </div>
    </footer>
  </section> -->

  <div class="chat-list">

  <div class="chat-row table-header">
    <span>Chats</span>
    <span>Chat ID</span>
    <span>Updated</span>
    <span></span>
  </div>

  {#if isLoading}
    <div class="empty-state">
      Loading conversations...
    </div>

  {:else if error}
    <div class="empty-state">
      <p>{error}</p>
      <button onclick={loadChats}>Retry</button>
    </div>

  {:else if chats.length === 0}
    <div class="empty-state">
      No conversations yet.
    </div>

  {:else}
    {#each chats as chat (chat.id)}
      <button
        type="button"
        class="chat-row"
        onclick={() => openChat(chat.id)}
      >
        <span class="chat-title">
          {chat.title}
        </span>

        <span class="chat-id">
          {chat.id}
        </span>

        <span class="chat-date">
          {formatDate(chat.updatedAt)}
        </span>

        <span class="chat-action">
          Open →
        </span>
      </button>
    {/each}
  {/if}

</div>
  <section class="workspace-note">
    <div class="note-icon">i</div>
    <div>
      <strong>Investigation workspace</strong>
      <p>
        Keep case records, evidence references, examination notes, and
        generated reports linked to the same investigation.
      </p>
    </div>
  </section>

  <footer class="page-footer">
    <span>Forensic Workspace</span>
    <span>Investigation management · Local workspace</span>
  </footer>
</div>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    background: #faf9f6;
    color: #292824;
    font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }

    .chat-list {
    width: 100%;
  }

  .chat-row {
    display: grid;
    grid-template-columns: 1fr 1.5fr 1fr auto;
    align-items: center;
    gap: 16px;
    width: 100%;
    padding: 14px 16px;
    border-bottom: 1px solid var(--border-color, #333);
    text-align: left;
  }

  .table-header {
    font-size: 12px;
    font-weight: 600;
    opacity: 0.65;
  }

  button.chat-row {
    background: transparent;
    color: inherit;
    border-top: none;
    border-left: none;
    border-right: none;
    cursor: pointer;
  }

  button.chat-row:hover {
    background: var(--chat-hover, rgba(128, 128, 128, 0.08));
  }

  .chat-title {
    font-weight: 600;
  }

  .chat-id {
    font-size: 12px;
    opacity: 0.55;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .chat-date {
    font-size: 13px;
    opacity: 0.7;
  }

  .chat-action {
    font-size: 13px;
    opacity: 0.7;
  }

  .empty-state {
    padding: 32px 16px;
    text-align: center;
    opacity: 0.65;
  }

  @media (max-width: 650px) {
    .chat-row {
      grid-template-columns: 1fr auto;
    }

    .table-header span:nth-child(2),
    .table-header span:nth-child(3),
    .chat-id,
    .chat-date {
      display: none;
    }
  }

  .investigations-page {
    width: 100%;
    margin: 0 auto;
    padding: 32px;
  }

  .page-header {
    margin-bottom: 28px;
  }

  .eyebrow {
    margin-bottom: 10px;
    color: #b4775c;
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 1.5px;
  }

  h1 {
    margin: 0;
    font-family: Georgia, "Times New Roman", serif;
    font-size: 34px;
    font-weight: 400;
    letter-spacing: -0.7px;
  }

  .page-header p {
    max-width: 560px;
    margin: 10px 0 0;
    color: #77746c;
    font-size: 13px;
    line-height: 1.8;
  }

  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 14px;
    margin-bottom: 24px;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 340px;
    max-width: 100%;
    padding: 10px 12px;
    border: 1px solid #e5e2db;
    border-radius: 7px;
    background: #fffefa;
    color: #89867d;
  }

  .search-box input {
    width: 100%;
    min-width: 0;
    border: 0;
    outline: none;
    background: transparent;
    color: #292824;
    font-size: 12px;
  }

  button {
    font: inherit;
    cursor: pointer;
  }

  .primary-button {
    padding: 10px 14px;
    border: 1px solid #34332e;
    border-radius: 7px;
    background: #34332e;
    color: #fffefa;
    font-size: 12px;
    white-space: nowrap;
  }

  .overview {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 14px;
    margin-bottom: 32px;
  }

  .stat-card {
    display: flex;
    flex-direction: column;
    gap: 9px;
    padding: 18px;
    border: 1px solid #e8e5de;
    border-radius: 9px;
    background: #fffefa;
  }

  .stat-card > span {
    color: #77746c;
    font-size: 12px;
  }

  .stat-card strong {
    font-size: 27px;
    font-weight: 500;
  }

  .stat-card small {
    color: #969187;
    font-size: 11px;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 14px;
  }

  .section-header h2 {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
  }

  .section-header p {
    margin: 5px 0 0;
    color: #77746c;
    font-size: 12px;
  }

  .filter-button {
    padding: 8px 11px;
    border: 1px solid #e5e2db;
    border-radius: 6px;
    background: #fffefa;
    color: #555249;
    font-size: 11px;
  }

  .investigation-list {
    overflow-x: auto;
    border: 1px solid #e8e5de;
    border-radius: 9px;
    background: #fffefa;
  }

  .investigation-row {
    display: grid;
    grid-template-columns: minmax(230px, 2fr) minmax(90px, 0.7fr)
      minmax(80px, 0.6fr) minmax(105px, 0.8fr)
      minmax(100px, 0.8fr) 28px;
    align-items: center;
    gap: 12px;
    min-width: 900px;
    padding: 14px 18px;
    border-bottom: 1px solid #efede7;
  }

  .investigation-row:last-child {
    border-bottom: 0;
  }

  .table-header {
    min-height: 42px;
    background: #f8f7f3;
    color: #858177;
    font-size: 10px;
    font-weight: 600;
  }

  .case-name {
    display: flex;
    align-items: center;
    gap: 11px;
    min-width: 0;
  }

  .case-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 36px;
    height: 36px;
    border: 1px solid #e7e1d7;
    border-radius: 6px;
    background: #f7f4ed;
    color: #8d725c;
    font-size: 10px;
    font-weight: 600;
  }

  .case-name strong {
    display: block;
    overflow: hidden;
    color: #393832;
    font-size: 12px;
    font-weight: 550;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .case-name small {
    display: block;
    margin-top: 5px;
    color: #89867d;
    font-size: 10px;
  }

  .case-id,
  .date {
    color: #69665e;
    font-size: 11px;
  }

  .priority,
  .status {
    justify-self: start;
    padding: 5px 8px;
    border-radius: 5px;
    font-size: 10px;
    white-space: nowrap;
  }

  .high {
    background: #f7e9e4;
    color: #a45d48;
  }

  .medium {
    background: #f8f0df;
    color: #97743a;
  }

  .low {
    background: #efeee9;
    color: #77746c;
  }

  .active {
    background: #edf1e8;
    color: #5d7652;
  }

  .review {
    background: #f8f0df;
    color: #97743a;
  }

  .completed {
    background: #edf3eb;
    color: #54744f;
  }

  .more-button {
    border: 0;
    background: transparent;
    color: #77746c;
    font-size: 19px;
  }

  .list-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-top: 14px;
    color: #89867d;
    font-size: 11px;
  }

  .pagination {
    display: flex;
    gap: 5px;
  }

  .pagination button {
    padding: 7px 9px;
    border: 1px solid #e5e2db;
    border-radius: 5px;
    background: #fffefa;
    color: #69665e;
    font-size: 10px;
  }

  .pagination button:disabled {
    opacity: 0.45;
    cursor: default;
  }

  .pagination .current-page {
    border-color: #34332e;
    background: #34332e;
    color: white;
  }

  .workspace-note {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    margin-top: 28px;
    padding: 18px;
    border: 1px solid #e8e2d6;
    border-radius: 9px;
    background: #f5f1e8;
  }

  .note-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 24px;
    height: 24px;
    border: 1px solid #dcd3c2;
    border-radius: 50%;
    color: #8d725c;
    font-size: 12px;
  }

  .workspace-note strong {
    font-size: 12px;
    font-weight: 600;
  }

  .workspace-note p {
    margin: 6px 0 0;
    color: #77746c;
    font-size: 11px;
    line-height: 1.8;
  }

  .page-footer {
    display: flex;
    justify-content: space-between;
    gap: 12px;
    margin-top: 36px;
    padding-top: 16px;
    border-top: 1px solid #e8e5de;
    color: #89867d;
    font-size: 10px;
  }

  @media (max-width: 650px) {
    .investigations-page {
      padding: 24px 16px;
    }

    .overview {
      grid-template-columns: 1fr;
      gap: 10px;
    }

    .stat-card {
      padding: 14px;
    }

    .toolbar {
      align-items: stretch;
      flex-direction: column;
    }

    .search-box {
      width: 100%;
    }

    .primary-button {
      align-self: flex-start;
    }

    .list-footer,
    .page-footer {
      align-items: flex-start;
      flex-direction: column;
    }
  }
</style>
