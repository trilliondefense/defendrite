<script lang="ts">
  type CaseStatus = 'Active' | 'Review' | 'Closed';

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
  <title>Cases | Digital Forensics</title>
</svelte:head>

<section class="cases-page">
  <header class="page-header">
    <div>
      <p class="eyebrow">INVESTIGATION MANAGEMENT</p>
      <h1>Cases</h1>
      <p class="description">
        Manage investigations, track evidence, and review case activity.
      </p>
    </div>

    <button class="primary-button" onclick={() => (showCreateForm = !showCreateForm)}>
      + New case
    </button>
  </header>

  <section class="stats">
    <article class="stat-card">
      <div class="stat-heading">
        <span>Total cases</span>
        <span class="stat-icon">▤</span>
      </div>
      <h2>{cases.length}</h2>
      <p>All investigations</p>
    </article>

    <article class="stat-card">
      <div class="stat-heading">
        <span>Active investigations</span>
        <span class="stat-icon green">◉</span>
      </div>
      <h2>{activeCases}</h2>
      <p>Currently in progress</p>
    </article>

    <article class="stat-card">
      <div class="stat-heading">
        <span>Pending review</span>
        <span class="stat-icon amber">◷</span>
      </div>
      <h2>{reviewCases}</h2>
      <p>Awaiting review</p>
    </article>

    <article class="stat-card">
      <div class="stat-heading">
        <span>Evidence records</span>
        <span class="stat-icon violet">⬡</span>
      </div>
      <h2>{evidenceCount}</h2>
      <p>Across all cases</p>
    </article>
  </section>

  {#if showCreateForm}
    <form
      class="create-form"
      onsubmit={(event) => {
        event.preventDefault();
        createCase();
      }}
    >
      <div class="form-heading">
        <div>
          <h2>Create investigation</h2>
          <p>Enter the basic details for the new case.</p>
        </div>

        <button
          type="button"
          class="close-button"
          onclick={() => (showCreateForm = false)}
        >
          ×
        </button>
      </div>

      <div class="form-grid">
        <label>
          Case title
          <input bind:value={title} required placeholder="Enter case title" />
        </label>

        <label>
          Investigation type
          <select bind:value={type}>
            <option>Computer Forensics</option>
            <option>Mobile Forensics</option>
            <option>Network Forensics</option>
            <option>Incident Response</option>
            <option>Malware Analysis</option>
            <option>Data Recovery</option>
            <option>Cloud Forensics</option>
          </select>
        </label>

        <label>
          Assigned investigator
          <input
            bind:value={investigator}
            required
            placeholder="Investigator name"
          />
        </label>
      </div>

      <div class="form-actions">
        <button
          type="button"
          class="secondary-button"
          onclick={() => (showCreateForm = false)}
        >
          Cancel
        </button>
        <button type="submit" class="primary-button">Create case</button>
      </div>
    </form>
  {/if}

  <section class="case-section">
    <header class="section-header">
      <div>
        <h2>Investigation register</h2>
        <p>Browse and manage forensic investigations.</p>
      </div>

      <div class="filters">
        <label class="search-box">
          <span>⌕</span>
          <input
            bind:value={search}
            placeholder="Search cases..."
            aria-label="Search cases"
          />
        </label>

        <select bind:value={statusFilter} aria-label="Filter by status">
          <option>All</option>
          <option>Active</option>
          <option>Review</option>
          <option>Closed</option>
        </select>
      </div>
    </header>

    <div class="table-wrapper">
      <table>
        <thead>
          <tr>
            <th>CASE / INVESTIGATION</th>
            <th>STATUS</th>
            <th>INVESTIGATOR</th>
            <th>EVIDENCE</th>
            <th>CREATED</th>
            <th></th>
          </tr>
        </thead>

        <tbody>
          {#each filteredCases as item (item.id)}
            <tr>
              <td>
                <div class="case-cell">
                  <div class="case-icon">▤</div>
                  <div>
                    <strong>{item.title}</strong>
                    <p>{item.reference} · {item.type}</p>
                  </div>
                </div>
              </td>

              <td>
                <span class={`status status-${item.status.toLowerCase()}`}>
                  <span class="status-dot"></span>
                  {item.status}
                </span>
              </td>

              <td>
                <div class="investigator-cell">
                  <span class="avatar">
                    {item.investigator
                      .split(' ')
                      .map((part) => part[0])
                      .join('')}
                  </span>
                  {item.investigator}
                </div>
              </td>

              <td>
                <span class="evidence-count">⬡ {item.evidence}</span>
              </td>

              <td>{item.createdAt}</td>

              <td>
                <button
                  class="row-button"
                  title="Case reference"
                  aria-label={`Reference ${item.reference}`}
                  onclick={() => navigator.clipboard?.writeText(item.reference)}
                >
                  ···
                </button>
              </td>
            </tr>
          {:else}
            <tr>
              <td colspan="6">
                <div class="empty-state">
                  <span>⌕</span>
                  <strong>No cases found</strong>
                  <p>Try changing your search or status filter.</p>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>

    <footer class="table-footer">
      <span>Showing {filteredCases.length} of {cases.length} cases</span>
      <span>Local session · Sample data</span>
    </footer>
  </section>
</section>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  .cases-page {
    width: 100%;
    min-width: 0;
    padding: 30px;
    color: #30342f;
    background: #fafaf8;
  }

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
    margin-bottom: 28px;
  }

  .eyebrow {
    margin: 0 0 8px;
    color: #85877f;
    font-size: 10px;
    letter-spacing: 1.2px;
  }

  h1 {
    margin: 0 0 8px;
    font-family: Georgia, serif;
    font-size: 34px;
    font-weight: 400;
    letter-spacing: -0.8px;
  }

  .description,
  .section-header p,
  .form-heading p {
    margin: 0;
    color: #7c7f77;
    font-size: 12px;
    line-height: 1.6;
  }

  button,
  input,
  select {
    font: inherit;
  }

  button {
    cursor: pointer;
  }

  .primary-button,
  .secondary-button {
    min-height: 36px;
    padding: 0 14px;
    border: 1px solid #405d49;
    border-radius: 6px;
    background: #405d49;
    color: white;
    font-size: 11px;
    white-space: nowrap;
  }

  .primary-button:hover {
    background: #304b39;
  }

  .secondary-button {
    border-color: #e3e2dc;
    background: white;
    color: #444840;
  }

  .stats {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 14px;
    margin-bottom: 32px;
  }

  .stat-card {
    min-width: 0;
    padding: 17px;
    border: 1px solid #e8e7e1;
    border-radius: 8px;
    background: white;
  }

  .stat-heading {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    color: #73766e;
    font-size: 11px;
  }

  .stat-icon {
    display: grid;
    width: 30px;
    height: 30px;
    flex-shrink: 0;
    place-items: center;
    border-radius: 7px;
    background: #f1f1ed;
    color: #74776f;
    font-size: 16px;
  }

  .stat-icon.green {
    background: #eaf1e9;
    color: #4b7855;
  }

  .stat-icon.amber {
    background: #f7f0e4;
    color: #a37c3c;
  }

  .stat-icon.violet {
    background: #efedf7;
    color: #7971a2;
  }

  .stat-card h2 {
    margin: 15px 0 8px;
    font-family: Georgia, serif;
    font-size: 32px;
    font-weight: 400;
  }

  .stat-card p {
    margin: 0;
    color: #85877f;
    font-size: 10px;
  }

  .case-section,
  .create-form {
    overflow: hidden;
    border: 1px solid #e8e7e1;
    border-radius: 8px;
    background: white;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
    padding: 18px;
  }

  .section-header h2,
  .form-heading h2 {
    margin: 0 0 6px;
    font-size: 14px;
    font-weight: 600;
  }

  .filters {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .search-box {
    display: flex;
    width: 230px;
    height: 35px;
    align-items: center;
    gap: 8px;
    padding: 0 10px;
    border: 1px solid #e3e2dc;
    border-radius: 5px;
    color: #85877f;
  }

  .search-box span {
    font-size: 19px;
  }

  .search-box input {
    width: 100%;
    min-width: 0;
    border: 0;
    outline: 0;
    background: transparent;
    font-size: 11px;
  }

  .filters select,
  .form-grid input,
  .form-grid select {
    min-height: 35px;
    padding: 8px 10px;
    border: 1px solid #e3e2dc;
    border-radius: 5px;
    background: white;
    color: #444840;
    font-size: 11px;
  }

  .table-wrapper {
    width: 100%;
    overflow-x: auto;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    text-align: left;
    white-space: nowrap;
  }

  thead {
    background: #f8f8f5;
  }

  th {
    height: 40px;
    color: #81837b;
    font-size: 9px;
    font-weight: 600;
    letter-spacing: 0.5px;
  }

  th,
  td {
    padding: 12px 15px;
    border-top: 1px solid #eeede8;
  }

  td {
    color: #555950;
    font-size: 11px;
  }

  .case-cell {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .case-icon {
    display: grid;
    width: 34px;
    height: 36px;
    flex-shrink: 0;
    place-items: center;
    border: 1px solid #e8e7e1;
    border-radius: 6px;
    background: #f8f8f4;
    color: #6d786c;
    font-size: 15px;
  }

  .case-cell strong {
    color: #30342f;
    font-size: 11px;
    font-weight: 600;
  }

  .case-cell p {
    margin: 5px 0 0;
    color: #85877f;
    font-size: 10px;
  }

  .status {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 5px 8px;
    border-radius: 5px;
    font-size: 10px;
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: currentColor;
  }

  .status-active {
    background: #edf5ec;
    color: #426f48;
  }

  .status-review {
    background: #faf3e5;
    color: #936d2d;
  }

  .status-closed {
    background: #f0f0ec;
    color: #70736c;
  }

  .investigator-cell {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .avatar {
    display: grid;
    width: 27px;
    height: 27px;
    flex-shrink: 0;
    place-items: center;
    border-radius: 50%;
    background: #f0f0eb;
    color: #62675e;
    font-size: 9px;
  }

  .evidence-count {
    color: #555950;
  }

  .row-button,
  .close-button {
    border: 0;
    background: transparent;
    color: #777a73;
    font-size: 17px;
  }

  .row-button:hover,
  .close-button:hover {
    color: #30342f;
  }

  .table-footer {
    display: flex;
    justify-content: space-between;
    gap: 10px;
    padding: 14px 17px;
    border-top: 1px solid #eeede8;
    color: #85877f;
    font-size: 10px;
  }

  .create-form {
    margin-bottom: 24px;
    padding: 20px;
  }

  .form-heading {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    margin-bottom: 18px;
  }

  .close-button {
    font-size: 23px;
  }

  .form-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 14px;
  }

  .form-grid label {
    display: grid;
    gap: 7px;
    color: #555950;
    font-size: 11px;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 9px;
    margin-top: 18px;
  }

  .empty-state {
    display: flex;
    min-height: 180px;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    gap: 8px;
    color: #85877f;
    white-space: normal;
    text-align: center;
  }

  .empty-state > span {
    font-size: 25px;
  }

  .empty-state strong {
    color: #30342f;
  }

  .empty-state p {
    margin: 0;
    font-size: 11px;
  }

  @media (max-width: 1050px) {
    .stats {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .form-grid {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 650px) {
    .cases-page {
      padding: 20px 14px;
    }

    .page-header,
    .section-header {
      align-items: flex-start;
      flex-direction: column;
    }

    .filters {
      width: 100%;
    }

    .search-box {
      flex: 1;
      width: auto;
    }

    .table-footer {
      flex-direction: column;
    }
  }
</style>