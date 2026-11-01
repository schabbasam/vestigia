
 // All texts, introductions, ASCII art, etc. live here.

pub const INTRO: &str = r#"
You are leisurely walking through some strawberry fields.

The sun hasn't even risen yet.

Enjoying the quiet, peaceful stride through the rural area,
you decide to climb a little mountain.

The weather is phenomenal, neither too cold nor unbearably hot.
Moonlight is more than enough to see everything.

Making full use of your free will, you are already at the foot
of the mountain. It only took an hour.

Leaning against the signal tower on the peak, you check the
smartphone in your pocket.

Suddenly, a thunderstorm starts. It rains a lot. Although it is
just water, you'd rather stay dry.

Lightning illuminates a little shed that you somehow overlooked
before.

Standing in front of the shabby wooden building, you decide
to step inside. It looks so run-down and abandoned that it
feels like no one would care about your presence.

The floor creaks. Just a big table and a pretty nice-looking
chair. That's all there is.

And only then do you notice the opening in the floor.

Stairs leading somewhere.

Surprisingly, it doesn't feel as haunting as one might imagine.

After two whole minutes of walking down the stairs, you reach
a big door.

It isn't locked.

What could possibly happen?

Without a second thought, you enter a big rectangular room.

The room is filled with shelves, cabinets, and all kinds of
things that seem to have been collected over the course of
several decades.

Old books sit next to peculiar instruments, boxes of unknown
contents, and objects that probably had a purpose at some
point.

In the middle of it all stands a desk. An old-looking computer
monitor rests on top of it, its bulky frame slightly out of
place among the surrounding clutter.

You have absolutely no idea what this place is supposed to be.

Well, you might as well take a look.

You pull out the chair and sit down.

The monitor displays a screensaver. A digital clock reads
02:56.

It is, after all, the middle of the night.

You press a key on the keyboard.

The screen flickers.

And then, something happens.
"#;


pub const WELCOME: &str = r#"
========================================
       NATSUKI PERSONAL TERMINAL
========================================

Welcome back.

System status: OPERATIONAL
User session: GUEST

Type 'help' to see available commands.
"#;

pub const SYSTEM_INFO: &str = r#"
SYSTEM INFORMATION
-----------------

Hostname: NATSUKI-01
Operating System: NATSUKI OS 4.2
Kernel: 4.2.17-retro
Architecture: x86_64

System status: OPERATIONAL
Last boot: 2026-10-28 08:42:16

This system is configured for personal
laboratory use.

Some directories may require authentication.
"#;

pub const PERSONNEL_RECORD: &str = r#"
PERSONNEL RECORD
----------------

Employee ID: 0471

First name: Daniel
Last name: Natsuki

Department: Applied Interface Research
Position: Lead Research Engineer

Date of employment: 2019-03-12
Clearance level: 3

Additional information:
Responsible for laboratory infrastructure,
experimental interface development and
system architecture.

Record last updated: 2026-09-18
"#;

pub const PERSONAL_NOTES: &str = r#"
PERSONAL NOTES
--------------

things to remember:

- order replacement parts for the lumen unit
- ask steve about the cabinet projector
- alicia still has the old access card
- lawliet wanted to discuss the latest
  orphues results

I should probably clean up this terminal
at some point.

Also, stop letting Steve change my wallpaper.
"#;

pub const ACCESS_MANUAL: &str = r#"
PERSONAL TERMINAL - ACCESS MANUAL
---------------------------------

Personal accounts follow a standardized
credential format.

USERNAME:
First name + "." + last name

PASSWORD:
Date of birth in DYDYMYMY format.

All credentials are case-sensitive.

Example:
John Doe, born 01.02.1990

Username: john.doe
Password: 01190920

"#;


// RESEARCH DOCUMENTS

pub const PROJECT_OVERVIEW: &str = r#"
PROJECT ORPHEUS
===============

Classification: Experimental
Department:     Applied Interface Research
Project lead:   Daniel Natsuki
Status:         ACTIVE

OVERVIEW

ORPHEUS aims to develop a fully adaptive
spatial interface between humans and
computing systems.

The system combines three major technologies:

1. ATLAS
   Real-time spatial mapping and environmental
   awareness.

2. ECHO
   Gesture recognition and operator tracking.

3. LUMEN
   Dynamic visual projection and holographic
   display technology.

Together, these systems allow an entire
environment to function as an interactive
computing interface.

Current objective:
Achieve seamless interaction without
traditional input devices.

Last revision: 2026-10-21
"#;

pub const EXPERIMENT_LOG: &str = r#"
EXPERIMENT LOG
==============

PROJECT: ORPHEUS

EXPERIMENT 001
--------------
Subject: Environmental mapping
System:  ATLAS

The system successfully mapped the entire
test environment and identified all objects.

Result: SUCCESS

EXPERIMENT 002
--------------
Subject: Operator recognition
System:  ECHO

The system correctly identified the operator
using a sequence of hand gestures.

Recognition accuracy: 99.7%

Result: SUCCESS

EXPERIMENT 003
--------------
Subject: Spatial projection
System:  LUMEN

Visual elements were projected onto multiple
surfaces simultaneously.

Result: SUCCESS

EXPERIMENT 004
--------------
Subject: Full system integration

All three subsystems operated simultaneously.

The environment responded to the operator
without requiring physical input devices.

Result: SUCCESS

Conclusion:
The system is ready for practical deployment.
"#;

pub const PROTOTYPE_NOTES: &str = r#"
PROTOTYPE DEVELOPMENT NOTES
===========================

ORPHEUS currently consists of three
independent subsystems.

ATLAS:
Uses spatial sensors to understand the
surrounding environment.

ECHO:
Interprets the operator's movements and
recognizes predefined gestures.

LUMEN:
Produces visual output based on information
received from the other systems.

INTEGRATION NOTES

The systems must be initialized in the
following order:

First, the environment must be understood.

Then, the operator must be identified.

Finally, visual output can be generated.

Do not initialize the systems manually
unless the automatic sequence fails.

- Daniel
"#;

pub const INTERNAL_REPORT: &str = r#"
INTERNAL PROJECT REPORT
=======================

Date:   2026-10-25
Author: Daniel Natsuki

ORPHEUS has reached its final development
stage.

All three subsystems are operational.
The latest integration test was successful.

The main remaining issue is the physical
installation.

Steve has finished relocating the projection
hardware. Alicia has completed the interface
calibration.

Lawliet is currently reviewing the final
test results.

The system is ready for demonstration.

One thing remains:

We need enough space for all four of us
during the next presentation.

By the way, the code is PHANTOM.

- Daniel
"#;


// MAINTENANCE DOCUMENTS

pub const MAINTENANCE_LOG: &str = r#"
MAINTENANCE LOG
===============

2026-10-20
----------
ATLAS sensors recalibrated.
Status: OK

2026-10-22
----------
ECHO gesture recognition updated.
Status: OK

2026-10-24
----------
LUMEN projection unit relocated.
Status: OK

2026-10-26
----------
Main terminal software updated.
Status: OK

2026-10-28
----------
Projection surface calibration completed.
Status: OK

Technician notes:

The projection unit is now installed inside
the storage cabinet.

All connections are hidden behind the
rear panel.

Do not disconnect the cabinet's power supply.

- Steve
"#;

pub const EQUIPMENT_INVENTORY: &str = r#"
EQUIPMENT INVENTORY
===================

MAIN TERMINAL
-------------
Type:       Personal workstation
Interface:  CRT-style display
Status:     OPERATIONAL

ATLAS
-----
Type:   Spatial sensor array
Status: OPERATIONAL

ECHO
----
Type:   Gesture recognition system
Status: OPERATIONAL

LUMEN
-----
Type:   Projection system
Status: OPERATIONAL

ADDITIONAL EQUIPMENT
--------------------

- Environmental lighting controller
- Holographic projection unit
- Spatial tracking cameras
- Wireless interface modules
- Auxiliary processing units

All equipment is currently operational.

Last inventory: 2026-10-28
"#;


// EMAILS

pub const MAIL_BIRTHDAY: &str = r#"
From:       alicia@jubel.lab
To:         daniel@jubel.lab
Date:       2026-06-14
Subject:    Happy Birthday

Happy birthday, Daniel! Did you really think
I would let this day pass quitly?
You are 32 already. That's crazy.

Lawliet said he has a surprise for you,
but knowing him, it is probably just another
ridiculously complicated research paper.

Have a great day!

- Alicia
"#;

pub const MAIL_CABINET: &str = r#"
From:       steve@jubel.lab
To:         daniel@jubel.lab
Date:       2026-10-24
Subject:    The cabinet

Hey Daniel,

The projector installation is finally done.

I honestly don't understand why you want
to hide such expensive hardware inside
an old wooden cabinet.

But whatever. It works.

Just don't put any more coffee on top of it.

- Steve
"#;

pub const MAIL_ORPHEUS: &str = r#"
From:       lawliet@jubel.lab
To:         daniel@jubel.lab
Date:       2026-10-26
Subject:    ORPHEUS - Final results

Daniel,

I reviewed the latest integration results.

ATLAS, ECHO and LUMEN are functioning
exactly as intended.

I still believe the gesture recognition
could be improved, but the current results
are sufficient for the demonstration.

One question:

Are you planning to show the system in
its default configuration?

I think the others would appreciate seeing
what it can actually do.

- Lawliet
"#;

pub const MAIL_DEMONSTRATION: &str = r#"
From:       daniel@jubel.lab
To:         alicia@jubel.lab
Date:       2026-10-27
Subject:    Tomorrow's demonstration

Hi Alicia,

Everything should be ready by tomorrow.

Steve finished the hardware installation
and Lawliet approved the final results.

I also prepared the other workspace.
There should be enough room for everyone now.

Please make sure everyone arrives on time.

And remind Steve not to touch my terminal
again.

- Daniel
"#;


// SYSTEM LOGS

pub const SYSTEM_LOG: &str = r#"
SYSTEM LOG
==========

2026-10-28 08:42:16
System boot completed.

2026-10-28 08:42:19
All services initialized.

2026-10-28 08:42:22
ATLAS: Operational.

2026-10-28 08:42:23
ECHO: Operational.

2026-10-28 08:42:24
LUMEN: Operational.

2026-10-28 08:42:25
ORPHEUS integration: Ready.

2026-10-28 08:43:01
User session initialized.

2026-10-28 08:43:02
Authentication level: Guest.

2026-10-28 08:43:02
Restricted directories locked.
"#;

pub const ACCESS_LOG: &str = r#"
ACCESS LOG
==========

2026-10-27 14:02:11
daniel.natsuki accessed /research/

2026-10-27 14:05:32
daniel.natsuki accessed /maintenance/

2026-10-27 14:08:44
steve accessed /maintenance/

2026-10-27 14:12:09
alicia accessed /research/

2026-10-28 08:42:16
System restarted.

2026-10-28 08:42:25
Guest session initialized.

2026-10-28 08:42:25
Access to /archive/ denied.
"#;

// ARCHIVE

pub const ARCHIVE_INDEX: &str = r#"
FACILITY ARCHIVE
================

Classification: INTERNAL
Access level: RESTRICTED

Available documents:

- facility_map.dat
- room_config.dat
- infrastructure.dat

NOTICE:

The architectural layout was last updated
after the laboratory expansion.

For security reasons, this archive is
restricted to authorized personnel.

Project reference: ORPHEUS

End of index.
"#;

pub const FACILITY_MAP: &str = r#"
FACILITY ARCHITECTURAL LAYOUT
=============================

                 FACILITY
+--------------------------------------+
|                                      |
|  +------------------+                |
|  |                  |                |
|  |     LAB 01       |                |
|  |                  |                |
|  |  Daniel          |                |
|  |                  |                |
|  +--------+---------+                |
|           |                          |
|           |  CONNECTION              |
|           |                          |
|  +--------+---------+                |
|  |                  |                |
|  |     LAB 02       |                |
|  |                  |                |
|  |  Alicia          |                |
|  |                  |                |
|  |                  |                |
|  +------------------+                |
|           |                          |
+--------------------------------------+

"#;

pub const OUTRO: &str = r#"
Is there really another hall just next door?

Well, according to the floor plan, there should be.

Curiosity gets the better of you, and you decide to follow the marked connection.

On your way there, something catches your attention.

The contents of the shelves don't actually seem all that old. Some of the objects look surprisingly well-maintained. A few even appear to be brand new.

Strange.

You eventually reach the door.

Unlike the massive entrance you came through earlier, this one is considerably lighter. Opening it takes almost no effort.

And what awaits you on the other side is certainly not what you expected.

A bright, remarkably modern laboratory stretches out before you. Clean surfaces, sophisticated equipment, enormous displays and all sorts of high-tech machinery.

Four people are sitting on chairs nearby, apparently rather busy discussing something.

You stand there for a moment, trying to make sense of what you're seeing.

Eventually, you decide to interrupt them.

You express your surprise and ask what this place is supposed to be.

Four friendly faces turn towards you.

They seem more amused than startled by your sudden appearance.

One of them begins explaining that this is the so-called "Jubel-Lab", a place where the four of them live while conducting various scientific research projects.

A rather young-looking guy takes the lead in the conversation and introduces himself.

His name is Daniel Natsuki.

You explain how you ended up here, starting with your little walk through the strawberry fields and ending with the discovery of the underground facility.

You also mention that you initially assumed the entire place was some sort of abandoned site.

The four of them exchange amused glances and chuckle.

One of the women finally points out that Daniel simply has a particular fondness for antique and retro-style aesthetics.

Well, that explains a few things.

After a little more conversation, you all decide to head back to the room you came from.

As you approach the desk, Daniel casually makes a few hand gestures.

A short chime rings through the room.

And suddenly, everything lights up.

The seemingly ancient computer monitor turns out to be a cutting-edge display. The walls come alive with countless screens, diagrams, graphs and other information you can barely begin to comprehend.

Even the shelves illuminate themselves. Digital name tags appear beside the objects, revealing their identities and presumably a lot more information.

The entire room transforms before your eyes.

Somehow, the combination of futuristic technology and retro aesthetics actually works surprisingly well.

You take a moment to appreciate the rather unusual sight.

After all, you didn't exactly expect to find a fully operational research facility beneath a seemingly abandoned shed.

You decide to stay a little longer.

There is still quite a lot to learn about this place, its inhabitants, and whatever it is they're working on.

And, more importantly, you've just met four rather interesting people.
"#;

