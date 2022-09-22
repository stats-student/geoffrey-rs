<p align="center">
    <img src="static/images/geoffrey-logo.png" class="logo">
</p>

<p align="center">
A simple tool to automate the creation of some folders and files for my
data science projects
</p>

<h3>Introduction</h3>
Geoffrey is a tool to automate and standardise(ish) the admin in my data science projects by creating folders and common files to speed up project setup and ensure that every project has a similar layout.
<br>
My general workflow for a data science project consists of 4 steps:
  
<ul style="list-style: none;">
  <li style="margin-bottom: 10px;">
    <img src="static/images/folder.png" height="25px" width="25px" style="vertical-align: middle; padding-right: 5px;">  Data sources 
  </li>
  <li style="margin-bottom: 10px;">
    <img src="static/images/magnifying-glass.png" height="25px" width="25px" style="vertical-align: middle;padding-right: 5px;"> Exploration
  </li>
  <li style="margin-bottom: 10px;">
    <img src="static/images/bar-chart.png" height="25px" width="25px" style="vertical-align: middle;padding-right: 5px;"> Models
  </li>
  <li>
    <img src="static/images/gift-box.png" height="25px" width="25px" style="vertical-align: middle;padding-right: 5px;"> Products
  </li>
</ul>

Geoffrey allows you to create projects and add in each of these 4 components in a modular way.

The quickstart is below and the manual for the different commands is <a href="docs/geoff.md">here</a>

<h3>Quickstart</h3>
<h5>Installation</h5>

```shell
foo@bar:~$ python -m pip install git+https://github.com/danielyates2/geoffrey#v0.1.1
Collecting git+https://github.com/danielyates2/geoffrey#v0.1.1
  Cloning https://github.com/danielyates2/geoffrey to /tmp/pip-req-build-3gtmwyf2
  Running command git clone -q https://github.com/danielyates2/geoffrey /tmp/pip-req-build-3gtmwyf2
  Installing build dependencies ... done
  ...
Successfully built geoffrey
```

<h5>Create a project</h5>

```shell
foo@bar:~$ geoff create test_project
🚀 test_project created!

test_project
├── 🖿 data_sources
├── 🖿 explorations
├── 🖿 models
├── 🖿 products
├── 🗋 README.md
└── 🗋 project_scoping.md
```

<h5>Add a data source</h5>
<h5>Add an exploration</h5>
<h5>Add a model</h5>
<h5>Add a product</h5>
