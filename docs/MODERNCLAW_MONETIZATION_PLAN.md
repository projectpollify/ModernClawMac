# ModernClaw Monetization Action Plan

## Purpose

This document is a step-by-step guide for turning ModernClaw from a working product into a revenue-generating product family.

It is written for a solo developer who is new to open sourcing and monetization. Every step is broken down in plain language.

The strategy is built on three revenue channels:

1. **Freemium (Open-Core)** — the main money engine
2. **Donations and Sponsorships** — early and supplementary income
3. **Crowdfunding** — an optional launch boost

---

## The Big Picture (Read This First)

Here is the simplest way to understand what we are doing:

You have one app right now called **ModernClawMulti**. It does a lot of powerful things. The plan is to split it into two products:

- **ModernClawBase** — a free, open-source version that does the core stuff really well. One brain, one voice, one model, chat, memory, knowledge. This is what you give away. It builds trust. It builds an audience. It gets people in the door.

- **ModernClawMulti** — the paid version. Everything Base does, plus multi-brain management, per-brain voices, per-brain models, wizard system, expert brain packs, automation, premium knowledge tools, and more. This is what you charge for.

The free version is not a broken demo. It is a real, useful product. People will use it, like it, and tell others about it. Some of those people will want the advanced version and pay for it. That is how open-core freemium works.

On top of that, you set up ways for people who love the project to donate money even before the paid version is ready. And optionally, you run a crowdfunding campaign to fund the first official release.

---

## Revenue Channel 1: Freemium (Open-Core)

This is your primary revenue source. Everything else is supplementary.

### What "Open-Core" Means

Open-core means:

- The core product is free and open source (ModernClawBase)
- The advanced product is paid and proprietary (ModernClawMulti)
- Both share the same foundation
- The free version is good enough to stand on its own
- The paid version is clearly worth paying for

Companies like GitLab, Grafana, and Sentry use this model successfully.

### The Product Split

This split has already been designed in the Base Split Plan. Here is the summary:

#### ModernClawBase (Free, Open Source)

What users get for free:

- Single-brain chat experience
- Conversation history
- Editable SOUL.md, USER.md, and MEMORY.md
- Daily logs
- Knowledge file ingestion
- Memory view
- Brain view
- Settings view
- Ollama integration with Gemma 4 baseline
- Basic voice support (Piper + Whisper)
- Speech normalization
- Clean onboarding
- Curator staging (if kept in core)

#### ModernClawMulti (Paid)

What users pay for:

- Everything in Base, plus:
- Multiple brains (create, switch, delete)
- Per-brain conversations, memory, and knowledge isolation
- Per-brain model preferences
- Per-brain voice identity
- Support brain as a dedicated product specialist
- Wizard system for guided brain creation
- Advanced knowledge workflows
- Expert brain packs (customer support, marketing, founder packs)
- Workspace organization (rename, pin, folder, archive, search, star)
- Brain management (archive, clone, template, export/import, snapshots)
- Premium voice packs
- Automation layer (recurring summaries, scheduled refresh)
- Future business and team layers

### Pricing Strategy

Here is the recommended approach for your first paid product:

#### Option A: One-Time Purchase (Recommended to Start)

- Price: **$29–$49 one-time** for ModernClawMulti
- Why: Easiest to set up, easiest for customers to understand, no subscription fatigue
- Works well for desktop apps that run locally
- You can always add subscriptions later for premium packs

#### Option B: Subscription (Later)

- Price: **$5–$10/month** or **$49–$99/year**
- Why: Recurring revenue is more predictable
- Better suited once you have add-on packs (expert brains, voice packs, automation)
- Harder to justify early when the product is still growing

#### Recommendation

Start with Option A (one-time purchase). It is simpler, feels fairer to users, and gets money in the door faster. Add subscription tiers later when you have enough premium content to justify ongoing payments.

### How to Actually Sell It (Payment Infrastructure)

You need a way to collect money and deliver the product. Here are the simplest options:

#### For Desktop App Sales

**Gumroad** (simplest to start)

- What it is: A platform where you can sell digital products
- How it works: You upload your installer file, set a price, and get a link to share
- They handle payments, taxes, and delivery
- Fee: 10% of each sale
- Setup time: About 1 hour
- Steps:
  1. Go to gumroad.com and create an account
  2. Click "New Product"
  3. Upload your ModernClawMulti Windows installer
  4. Set the price
  5. Write a short product description
  6. Publish
  7. Share the link on your GitHub README, website, and social media

**Paddle** (better for software)

- What it is: A payment platform designed for software products
- Handles global taxes automatically (this matters — tax compliance is real)
- Fee: 5% + $0.50 per transaction
- More professional feel
- Supports license keys
- Setup time: A few hours to a few days (they review your account)

**LemonSqueezy** (good middle ground)

- What it is: Similar to Gumroad but designed more for software and SaaS
- Handles tax compliance globally
- Supports license keys, subscriptions, and one-time purchases
- Fee: 5% + $0.50 per transaction
- Clean checkout experience
- Setup time: About 1–2 hours

#### Recommendation

Start with **Gumroad** if you need money coming in this week. Move to **LemonSqueezy** or **Paddle** once you want license keys and a more professional setup.

### License Key System

Once you move past simple file downloads, you will want license keys so that:

- Only paying customers can use ModernClawMulti
- You can verify purchases
- You can offer upgrades and renewals later

This is a future step. For now, selling the installer directly through Gumroad works. When you are ready, LemonSqueezy and Paddle both have built-in license key systems.

---

## Revenue Channel 2: Donations and Sponsorships

This is money you can start earning right now, before the paid product is even ready.

### Why Donations Work for Open Source

People who use free open source software and find it genuinely useful often want to support the developer. Donations are not charity — they are a way for users to say "keep going, this is valuable."

You will not get rich from donations alone, but they can cover basic costs and provide a financial bridge while you build toward the paid product.

### Platforms to Set Up

#### GitHub Sponsors (Highest Priority)

- What it is: A way for people to send you monthly or one-time payments directly through GitHub
- Why it matters: Your code lives on GitHub, so this is where developers already are
- GitHub takes no fee (they used to match donations too, check if that is still active)
- Steps:
  1. Go to github.com/sponsors
  2. Apply for the Sponsors program (takes a few days to get approved)
  3. Set up tiers (suggested: $3/mo, $7/mo, $15/mo, $30/mo)
  4. Write a short sponsor profile explaining what the money supports
  5. Add a "Sponsor" button to your GitHub repo

**Suggested Tier Names and Descriptions:**

| Tier | Price | Description |
|------|-------|-------------|
| Supporter | $3/mo | You believe in local-first AI and want to help |
| Backer | $7/mo | You use ModernClaw and want to see it grow |
| Builder | $15/mo | You want early access to Multi features and a say in the roadmap |
| Champion | $30/mo | You want direct input and priority support |

#### Ko-fi (Quick Setup, Low Commitment)

- What it is: A simple "buy me a coffee" style donation page
- Why it matters: Very low friction for one-time donations
- No monthly fee to you, they take a small cut only on certain features
- Steps:
  1. Go to ko-fi.com and create an account
  2. Customize your page with your ModernClaw branding
  3. Share the link anywhere (README, social media, app About screen)

#### Open Collective (If You Want Transparency)

- What it is: A platform for transparent community funding
- Shows exactly how money comes in and how it is spent
- Good for building trust with an open source community
- Best once you have a meaningful community around the project

#### Recommendation

Set up **GitHub Sponsors** and **Ko-fi** this week. They take very little time and immediately give people a way to support you. Add Open Collective later if the community grows.

### Where to Put Donation Links

Put your donation and sponsorship links everywhere people encounter your project:

- GitHub README (near the top, with a "Support This Project" section)
- GitHub repo sidebar (Sponsor button)
- ModernClawBase app itself (a small "Support" link in Settings or About)
- Your personal website or landing page if you have one
- Social media profiles

---

## Revenue Channel 3: Crowdfunding (Optional Launch Boost)

Crowdfunding is a one-time campaign to raise money for a specific goal, like shipping the first official public release of ModernClaw.

### When to Do This

Do not crowdfund immediately. Crowdfunding works best when you already have:

- A working demo or prototype (you have this)
- A clear story about what the product does and why it matters
- At least a small audience (even 100 interested people is enough to start)
- A specific goal ("fund the first public release" or "fund macOS and Linux support")

### Platforms

#### Kickstarter

- Best for products with a clear launch story
- All-or-nothing funding (you set a goal; if you do not hit it, nobody is charged)
- Good for generating buzz and press
- Takes about 5% fee plus payment processing

#### Indiegogo

- More flexible than Kickstarter (you can keep funds even if you do not hit your goal)
- Less mainstream attention than Kickstarter
- Similar fee structure

#### GitHub Sponsors (One-Time Goals)

- You can set funding goals on your GitHub Sponsors profile
- Less dramatic than a Kickstarter campaign but lower effort
- Good for smaller goals like "fund Linux support" or "fund a new voice pack"

### Recommended Campaign Structure

If you decide to crowdfund, here is a simple structure:

**Campaign Title:** "Help Ship ModernClaw — Local AI Brains You Actually Own"

**Goal:** $2,000–$5,000 (keep it reachable)

**What the Money Funds:**

- Polish and package ModernClawBase for public release
- Build the first official ModernClawMulti paid release
- Add macOS support
- Cover basic infrastructure costs (domain, distribution, code signing)

**Reward Tiers:**

| Tier | Price | Reward |
|------|-------|--------|
| Supporter | $5 | Name in the credits, early access to Base |
| Backer | $15 | Free copy of ModernClawMulti when it launches |
| Builder | $30 | Free copy + early access to premium brain packs |
| Founding Patron | $75 | All of the above + input on the roadmap + lifetime updates |

### Recommendation

Hold off on crowdfunding until you have completed Phase 1 and Phase 2 below. By then you will have a public repo, a landing page, some early users, and donation infrastructure — all of which make a crowdfunding campaign dramatically more likely to succeed.

---

## The Action Plan: What to Do and When

### Phase 0: Immediate (This Week)

These are things you can do right now that cost nothing and take very little time.

**Step 1: Set Up GitHub Sponsors**

1. Go to github.com/sponsors
2. Apply for the program
3. Create your tiers (use the suggestions above)
4. Write a sponsor profile: who you are, what ModernClaw is, what the money supports

**Step 2: Set Up Ko-fi**

1. Go to ko-fi.com
2. Create your page
3. Add your ModernClaw description and a link to the repo

**Step 3: Add a FUNDING.yml to Your Repo**

This tells GitHub to show a "Sponsor" button on your repo page.

1. In your repo, create a file at `.github/FUNDING.yml`
2. Put this in it:

```yaml
github: YOUR_GITHUB_USERNAME
ko_fi: YOUR_KOFI_USERNAME
```

3. Commit and push. The Sponsor button will appear on your repo.

**Step 4: Add a "Support This Project" Section to Your README**

Add a short section near the top of your README that says something like:

> ModernClaw is built by a solo developer. If this project is useful to you, please consider supporting it through GitHub Sponsors or Ko-fi. Every contribution helps keep development going.

Link to both platforms.

**Step 5: Pick Your Payment Platform**

Decide between Gumroad (fastest) or LemonSqueezy (more professional) for selling ModernClawMulti later. You do not need to set it up yet, just create an account so it is ready.

---

### Phase 1: Foundation (Weeks 1–4)

This phase is about getting the free product out the door and building an audience.

**Step 1: Create the ModernClawBase Repo**

Follow the technical split strategy from the Base Split Plan:

1. Create a new repo called `ModernClawBase`
2. Copy the current ModernClawMulti codebase into it
3. Remove the multi-brain UI (brain selector, create/switch/delete)
4. Simplify settings for a single-brain experience
5. Keep the mature workspace logic, voice behavior, and Gemma-first defaults
6. Rewrite the README for the Base identity

**Step 2: Write a Clear README for Base**

Your README is your storefront. It needs to explain:

- What ModernClaw is (in one sentence)
- What it does (in a short list)
- How to install it
- How to run it
- Where to get help
- How to support the project (donation links)
- That a more advanced paid version exists (ModernClawMulti)

**Step 3: Create a Simple Landing Page**

You need a single web page that explains what ModernClaw is to non-developers. This does not need to be fancy. Options:

- A simple page on GitHub Pages (free)
- A one-page site on Carrd.co ($19/year)
- A Notion page made public (free)

The page should have:

- Product name and one-line description
- A few screenshots or a short demo video
- Download link for ModernClawBase
- Link to the paid version (when ready)
- Donation/sponsor links

**Step 4: Package ModernClawBase for Download**

Right now the app requires a developer setup (Node, Rust, Ollama, manual Piper/Whisper install). For a public release, you need a downloadable installer.

You already have a working `npm run tauri:build` for Windows and a GitHub Actions release workflow. The steps are:

1. Complete the items on your Release Checklist
2. Tag a version in Git (e.g., `v0.1.0`)
3. Push the tag to trigger the GitHub Actions release workflow
4. The workflow builds Windows, macOS, and Linux installers
5. Attach the installers to a GitHub Release

Start with Windows only since that is validated. macOS and Linux can come later.

**Step 5: Write an Install Guide for Non-Developers**

Many potential users will not know what Ollama is or how to install Rust. Write a simple guide that walks them through everything:

1. Download the ModernClaw installer
2. Install Ollama (link, screenshots)
3. Pull the Gemma 4 model in Ollama
4. Install Piper and Whisper (if they want voice)
5. Launch ModernClaw

---

### Phase 2: First Revenue (Weeks 4–8)

This phase is about getting money coming in.

**Step 1: Prepare ModernClawMulti for Sale**

1. Make sure Multi is stable and fully working on Windows
2. Add a clear "this is the paid edition" indicator somewhere in the app
3. Decide on your price (recommended: $29–$39 one-time to start)
4. Build the Windows installer for Multi

**Step 2: Set Up Your Store**

1. Go to your chosen platform (Gumroad or LemonSqueezy)
2. Create a product listing for ModernClawMulti
3. Upload the installer
4. Write the product description:
   - What Multi adds over Base
   - Who it is for
   - What they get
5. Set the price
6. Publish

**Step 3: Connect Base to Multi**

In the ModernClawBase app and README, add clear but non-annoying references to the paid version:

- A small "Upgrade to Multi" link in Settings
- A mention in the README: "Need multiple brains? Check out ModernClawMulti"
- A comparison table on your landing page showing Base vs Multi features

The key principle: never make the free version feel crippled or naggy. Just make the upgrade path visible and easy.

**Step 4: Announce the Launch**

Write a launch post and share it on:

- Reddit (r/LocalLLaMA, r/selfhosted, r/opensource, r/artificial)
- Hacker News (Show HN post)
- Twitter/X
- Any AI or local-first communities you are part of
- Product Hunt (free to submit)

Your post should tell a story: who you are, why you built this, what makes it different (local-first, privacy-respecting, brain-building not just chatting), and how people can try it or support it.

---

### Phase 3: Growth (Months 2–6)

This phase is about growing the audience and revenue.

**Step 1: Build Premium Add-On Packs**

Once Multi is selling, start building the add-on packs from the split plan:

- **Workspace Organization Pack** (rename, pin, folder, archive, search, star)
- **Brain Management Pack** (archive, clone, template, export/import, snapshots)
- **Expert Brain Packs** (customer support template, marketing template, founder pack)
- **Premium Voice Packs** (additional curated Piper voices)

Sell these as add-ons or bundle them into Multi at a higher price tier.

**Step 2: Consider a Crowdfunding Campaign**

By now you should have:

- A public repo with some stars and users
- A landing page
- Some early sales
- A small community

This is the right time for a Kickstarter or Indiegogo campaign if you want a bigger push. Fund something specific like "macOS and Linux support" or "the wizard system."

**Step 3: Grow the Community**

- Start a Discord server for ModernClaw users
- Respond to GitHub issues and discussions
- Share development updates publicly
- Collect feature requests from paying customers
- Build in public (share progress on social media)

**Step 4: Explore Additional Revenue**

Once you have a stable product and growing audience, consider:

- **Consulting/custom brain building** — charge businesses to set up ModernClaw for their specific use case
- **Training content** — paid video tutorials or guides on brain building
- **Enterprise tier** — team features, shared brains, business support

---

## Quick Reference: Revenue Timeline

| When | What | Expected Revenue |
|------|------|-----------------|
| This week | GitHub Sponsors + Ko-fi | $0–$50/mo (grows over time) |
| Weeks 1–4 | ModernClawBase public release | $0 (audience building) |
| Weeks 4–8 | ModernClawMulti on sale | First sales ($100–$500/mo) |
| Months 2–4 | Launch posts, Product Hunt, Reddit | Growth ($500–$2,000/mo) |
| Months 3–6 | Add-on packs, community growth | Scaling ($1,000–$5,000/mo) |
| Month 6+ | Crowdfunding, enterprise, consulting | Variable |

These numbers are realistic estimates for a solo developer with a good product and active marketing. They are not guarantees. The actual numbers depend on how much time you put into marketing and community building alongside development.

---

## The One Rule That Matters Most

**Do not charge for trust. Charge for power.**

The free version must be genuinely useful. People need to trust it before they will pay for the advanced version. If the free version feels like a trap or a teaser, people will not trust the paid version either.

The paid version should make people think: "I love the free version, and the paid version gives me things I actually want." That is the feeling that makes open-core work.

---

## What to Do Right Now

If you are reading this and want to take action today, do these three things:

1. **Apply for GitHub Sponsors** (takes 5 minutes to apply, a few days to get approved)
2. **Create a Ko-fi page** (takes 10 minutes)
3. **Add a FUNDING.yml to your repo** (takes 2 minutes)

That is it. Those three things put donation infrastructure in place immediately while you work on everything else.

---

## Document History

- Created: April 2026
- Status: Active plan
- Author: Product development partnership (you + Claude)
- Next review: After Phase 1 completion
