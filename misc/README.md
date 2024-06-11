<div align="center">
  </br>
  <p>
    <img height="300" src="" />
  </p>
  <p>
    <strong>Tales of Sin'dor</strong>
  </p>
  <p>
    <strong>Program Architecture</strong>
  </p>
</div>

#

## World
- Create
- Update

## Character
- Create
- Update
- Delete

## Skill
- Create
- Update

## Resource
- Create
- Update
- Delete



## `Realm` 

- The `Realm Program` is responsible for:
  - Defining `Realm`s which players "connect" to;
    - Requiring players to "connect" to a `Realm` in order to play. This greatly increases client side scalability through phasing and lazy loading;
  - Providing a mechanism to disable certain types of and/or specific content/actions;

## `World` 

- The `World Program` is responsible for:
  - Defining `World`s which `Character`s exist in;
  - Providing a mechanism to disable locations within the world;

### `Item`  

  - The `Item Program` is resposible for:
    - Defining `Item`s which other program's derived addresses are authorized to create and destroy on-demand;

## `Combat` 

- The `Combat Program` is responsible for:
  - Defining `Attack`s which `Character`s and `Monster`s can perform;
  - Defining and providing mechanisms to empower the attacks;
  - Providing mechanisms to disable and/or change configuration value(s);

### `Ability` 

- The `Ability Program` is responsible for:
  - Defining `Ability`s which the `Item` and `Combat` programs use for combat;
  - Defining and providing mechanisms to empower the abilities;
  - Providing mechanisms to disable and/or change configuration value(s);


## `Character` 

- The `Character Program` is responsible for defining and representing an in-game `Character`.
- Allows customization of `Character` properties:
  - Name;
  - Appearance (Basic);
- Provides an `Inventory` which is expandable up to X (TBD) pages of Y (TBD) size based on `Character` level.


## `Skill` 

- The `Skill Program` is responsible for:
  - Defining `Skill`s, their experience tables, their content, etc...;
  - Representing a `Character`'s progress for a certain `Skill`;
  - A `Character`'s progress in a certain `Skill` can be increased or decreased by the `World`;
    - The `Character` performs an action on a certain `World`'s object and once that action has ended, it may yield experience (skill progress);
  - 