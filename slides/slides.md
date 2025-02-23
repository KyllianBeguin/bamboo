---
# You can also start simply with 'default'
theme: seriph
background: ./media/bamboo_background.jpg
# Informations about this presentation
title: Bamboo 🎍
info: Bamboo presentation
class: text-center
drawings:
  persist: false

transition: fade-out

# enable MDC Syntax: https://sli.dev/features/mdc
mdc: true
hideInToc: true
---

# Bamboo 🎍
Configuration-based data generation

<div @click="$slidev.nav.next" class="mt-12 py-1" hover:bg="white op-10">
  Let's go! <carbon:arrow-right />
</div>

<!--
Bienvenue à cette présentation ! Je vais vous parler de Bamboo, un outil permettant de configurer sa génération de jeu de données !
-->

---
transition: fade-out
hideInToc: true
---

# Summary

<Toc text-sm minDepth="1" maxDepth="2" />

---

# 🎍 About
<br/>
<br/>
<br/>

<h2>Bamboo is about <b>configuring</b> your data generation!</h2>
<br/>

```mermaid
flowchart LR
    A[/Config file 📄/] --> B([Bamboo 🎍])
    B --> C[(Dataset)]
```
---
transition: None
---

# 🤔 Why?
*A bit of storytelling*
<br/>
<br/>

During Machine Learning classes, we needed data to practice...  
Teacher(s) asked us to go for **data scraping**...  

<v-click><img src="./media/students_happy.jpg" width="50%"></v-click>

---
hideInToc: true
---

<style>
    .grid-container {
	    position: absolute;
		top: 50%;
		left: 50%;
        transform: translate(-50%, -50%);
        place-items: center;
    }
</style>

<div class="grid-container"><img src="./media/more_web_scraping.png"></div>

---
hideInToc: true
---

# 🤔 Why?
*A bit of storytelling*

<h2> Also web scraping is kinda <del>illegal</del> <b>risky</b></h2><i>(hello robot.txt)</i>

More information here : [Is web scraping legal? A short guide on scraping under EU law - Patrycja Szwed, 2021](https://discoverdigitallaw.com/is-web-scraping-legal-short-guide-on-scraping-under-the-eu-jurisdiction/)

---
hideInToc: true
---

# 🤔 Why?
<br/>
<br/>

<h2>🤯 No more <u>web scraping</u>!</h2>

<br/>

<h2>🤩 Work on <u>true use case</u> data</h2>

<br/>

<h2>😎 Generate data with your own <u>expectations</u></h2>

---

# 💥 How it works?

foo

---

# 🗺  Roadmap

foo

---

# Sprout you data, but under control !
