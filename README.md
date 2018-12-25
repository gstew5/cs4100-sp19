# Formal Languages and Compilers 

## Spring 2019

An upper-level course for CS majors on formal languages theory and compilers. 

Topics (subject to revision): regular expressions; finite automata; context-free grammars; predictive parsing; LR parsing; abstract syntax; type systems and type-checking; stack layout and activation records; intermediate representations; control-flow graphs; static-single assignment (SSA) form; dataflow/liveness analysis; register allocation; garbage collection/runtimes; the LLVM compiler infrastructure. Over the course of the semester, students will implement a full functioning compiler for a small imperative programming language, targeting LLVM. The course involves a significant amount of programming.

|                       |         Details      |
|-----------------------|----------------------|
| **Lecture**           | MWF |
| **Instructor**        | Gordon Stewart (gstewart@ohio.edu) |
| **Office Hours**      | TBD |
| **TA**                | Tim Steinberger |
| **Lab Hours**         | TBD |

## Texbook

TBD

## Course Difficulty

This is a demanding course that requires extensive programming work, in the form of a series of (often increasingly) difficult assignments. Expect to put in at least 10 hours (sometimes much more) per programming assignment.

## Course Structure

The course consists of weekly lectures (MWF), attendance at which is required. To help get you up to speed with the course programming assignments, we'll also hold biweekly lab hours (time TBD). Although attendance at the lab hours is optional, I highly recommend that you attend — at least for the first few weeks of the course. The programming assignments for this course are extensive and time consuming, so be prepared!

In addition to biweekly homework assignments, there will be a midterm exam (Week 7, approximately 15% of your grade) and a final (approximately 25%). The biweekly homeworks (programming assignments) are worth approximately 40%. We'll have weekly quizzes every Tuesday (with probability 1/3), along with bi-weekly offline Blackboard quizzes (total 10%). You get an additional 10% for free, just for signing up for the course.

### Grade Breakdown

| Component               | Percentage |
|-------------------------|-----|
| Programming assignments | 40% |
| Quizzes                 | 10% |
| Midterm exam            | 15% |
| Final exam              | 25% |
| Free points             | 10% |

Blackboard will be used only to report grades and to post lecture notes. Up-to-date information on all other aspects of the course (assignment due dates, etc.) will be posted either on this website or on the Piazza page or both. 

## Schedule

The schedule is subject to revision.

| Week                        | Topic                                 | Reading                        |
|-----------------------------|---------------------------------------|--------------------------------|
| Week 1 (14 Jan)             | Intro. to the course, compilers, Rust | [The Rust Book](https://doc.rust-lang.org/book/index.html) Chs. 1-3 |
| Week 2 (21 Jan)             | Rust contd. | [The Rust Book](https://doc.rust-lang.org/book/index.html) Chs. 4-6, 8 |
| Week 3 (28 Jan)             | NO CLASS (Stewart away) | |
| Week 4 (4 Feb)              | Virtual machines, bytecode, assemblers | |
| Week 5 (11 Feb)             | Regular expressions, finite automata, lexers | |
| Week 6 (18 Feb)             | From regexps to peephole optimizers | |
| Week 7 (25 Feb)             | Runtimes, garbage collection | |
| Week 8 (4 Mar)              | Intermediate representations | |
| Week 9 (11 Mar)             | SPRING BREAK | |
| Week 10 (18 Mar)            | Parsing | |
| Week 11 (25 Mar)            | From IR to bytecode | |
| Week 12 (1 Apr)             | Desugaring, the Grumpy source language | |
| Week 13 (8 Apr)             | Types and type checking | |
| Week 14 (15 Apr)            | Register allocation | |
| Week 15 (22 Apr)            | Slop | |
| 29 Apr - 3 May              | FINAL EXAM PERIOD | |
