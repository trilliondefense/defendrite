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

<div class="vault-page">
  <header class="page-header">
    <div class="eyebrow">FORENSIC WORKSPACE</div>
    <h1>Evidence Vault</h1>
    <p>
      Securely organize digital evidence, review integrity status, and track
      files associated with your investigations.
    </p>
  </header>

  <div class="toolbar">
    <label class="search-box">
      <span>⌕</span>
      <input
        type="search"
        placeholder="Search evidence..."
        aria-label="Search evidence"
      />
    </label>

    <button class="primary-button">+ Add evidence</button>
  </div>

  <section class="overview">
    <div class="stat-card">
      <span>Total evidence</span>
      <strong>128</strong>
      <small>Across all cases</small>
    </div>

    <div class="stat-card">
      <span>Verified</span>
      <strong>116</strong>
      <small>Integrity checks passed</small>
    </div>

    <div class="stat-card">
      <span>Storage used</span>
      <strong>8.4 GB</strong>
      <small>Of 50 GB available</small>
    </div>
  </section>

  <section class="vault-section">
    <div class="section-header">
      <div>
        <h2>Evidence records</h2>
        <p>Files preserved for forensic examination</p>
      </div>

      <button class="filter-button">All evidence⌄</button>
    </div>

    <div class="evidence-list">
      <div class="evidence-row table-header">
        <span>Evidence</span>
        <span>Case ID</span>
        <span>File size</span>
        <span>Integrity</span>
        <span></span>
      </div>

      <div class="evidence-row">
        <div class="evidence-name">
          <div class="file-icon">IMG</div>
          <div>
            <strong>device_backup.img</strong>
            <small>Disk image · EVD-00128</small>
          </div>
        </div>
        <span class="case-id">CASE-024</span>
        <span class="file-size">4.2 GB</span>
        <span class="status verified">Verified</span>
        <button class="more-button" aria-label="Evidence options">···</button>
      </div>

      <div class="evidence-row">
        <div class="evidence-name">
          <div class="file-icon">DB</div>
          <div>
            <strong>messages.sqlite</strong>
            <small>Database · EVD-00127</small>
          </div>
        </div>
        <span class="case-id">CASE-024</span>
        <span class="file-size">128 MB</span>
        <span class="status verified">Verified</span>
        <button class="more-button" aria-label="Evidence options">···</button>
      </div>

      <div class="evidence-row">
        <div class="evidence-name">
          <div class="file-icon">LOG</div>
          <div>
            <strong>system_events.log</strong>
            <small>Log file · EVD-00126</small>
          </div>
        </div>
        <span class="case-id">CASE-023</span>
        <span class="file-size">16.8 MB</span>
        <span class="status verified">Verified</span>
        <button class="more-button" aria-label="Evidence options">···</button>
      </div>

      <div class="evidence-row">
        <div class="evidence-name">
          <div class="file-icon">ZIP</div>
          <div>
            <strong>browser_artifacts.zip</strong>
            <small>Archive · EVD-00125</small>
          </div>
        </div>
        <span class="case-id">CASE-023</span>
        <span class="file-size">842 MB</span>
        <span class="status pending">Pending check</span>
        <button class="more-button" aria-label="Evidence options">···</button>
      </div>

      <div class="evidence-row">
        <div class="evidence-name">
          <div class="file-icon">PDF</div>
          <div>
            <strong>acquisition_notes.pdf</strong>
            <small>Document · EVD-00124</small>
          </div>
        </div>
        <span class="case-id">CASE-021</span>
        <span class="file-size">2.4 MB</span>
        <span class="status verified">Verified</span>
        <button class="more-button" aria-label="Evidence options">···</button>
      </div>
    </div>

    <footer class="list-footer">
      <span>Showing 5 of 128 evidence records</span>
      <div class="pagination">
        <button disabled>Previous</button>
        <button class="current-page">1</button>
        <button>2</button>
        <button>3</button>
        <button>Next</button>
      </div>
    </footer>
  </section>

  <section class="integrity-note">
    <div class="integrity-icon">✓</div>
    <div>
      <strong>Evidence integrity</strong>
      <p>
        Evidence integrity should be validated using cryptographic hashes.
        Original evidence should remain preserved throughout examination.
      </p>
    </div>
  </section>

  <footer class="page-footer">
    <span>Forensic Workspace</span>
    <span>Evidence storage · Local workspace</span>
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

  .vault-page {
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

  .evidence-list {
    overflow-x: auto;
    border: 1px solid #e8e5de;
    border-radius: 9px;
    background: #fffefa;
  }

  .evidence-row {
    display: grid;
    grid-template-columns: minmax(230px, 2fr) minmax(100px, 0.8fr)
      minmax(90px, 0.7fr) minmax(110px, 0.9fr) 28px;
    align-items: center;
    gap: 14px;
    min-width: 760px;
    padding: 14px 18px;
    border-bottom: 1px solid #efede7;
  }

  .evidence-row:last-child {
    border-bottom: 0;
  }

  .table-header {
    min-height: 42px;
    background: #f8f7f3;
    color: #858177;
    font-size: 10px;
    font-weight: 600;
  }

  .evidence-name {
    display: flex;
    align-items: center;
    gap: 11px;
    min-width: 0;
  }

  .file-icon {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 38px;
    border: 1px solid #e7e1d7;
    border-radius: 5px;
    background: #f7f4ed;
    color: #8d725c;
    font-size: 9px;
    font-weight: 700;
  }

  .evidence-name strong {
    display: block;
    overflow: hidden;
    color: #393832;
    font-size: 12px;
    font-weight: 550;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .evidence-name small {
    display: block;
    margin-top: 5px;
    color: #89867d;
    font-size: 10px;
  }

  .case-id,
  .file-size {
    color: #69665e;
    font-size: 11px;
  }

  .status {
    justify-self: start;
    padding: 5px 8px;
    border-radius: 5px;
    font-size: 10px;
    white-space: nowrap;
  }

  .verified {
    background: #edf3eb;
    color: #54744f;
  }

  .pending {
    background: #f8f0df;
    color: #97743a;
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

  .integrity-note {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    margin-top: 28px;
    padding: 18px;
    border: 1px solid #e8e2d6;
    border-radius: 9px;
    background: #f5f1e8;
  }

  .integrity-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 25px;
    height: 25px;
    border-radius: 50%;
    background: #e5e9dc;
    color: #54744f;
    font-size: 13px;
  }

  .integrity-note strong {
    font-size: 12px;
    font-weight: 600;
  }

  .integrity-note p {
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
    .vault-page {
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
