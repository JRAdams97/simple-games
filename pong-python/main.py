import esper
import pygame

FPS = 30
RESOLUTION = (800, 600)

PLAYER_WIDTH = 16
PLAYER_HEIGHT = 64
PLAYER_SPD = 8.0


# =====================================================================================================================
# COMPONENTS
# =====================================================================================================================
class Drawable:
    def __init__(self, is_drawable):
        self.is_drawable = is_drawable


class Position:
    def __init__(self, x=0.0, y=0.0, w=0.0, h=0.0):
        self.x = x
        self.y = y
        self.w = w
        self.h = h


class Velocity:
    def __init__(self, x=0.0, y=0.0):
        self.x = x
        self.y = y


# =====================================================================================================================
# SYSTEMS
# =====================================================================================================================
class DrawSystem(esper.Processor):
    def __init__(self, window, clear_color=(0, 0, 0)):
        super().__init__()
        self.window = window
        self.clear_color = clear_color

    def process(self):
        self.window.fill(self.clear_color)

        for e, (draw, pos) in self.world.get_components(Drawable, Position):
            if draw.is_drawable:
                pygame.draw.rect(self.window, (255, 255, 255), (pos.x, pos.y, pos.w, pos.h))

        pygame.display.flip()


class MovementSystem(esper.Processor):
    def __init__(self, min_x, max_x, min_y, max_y):
        super().__init__()

        self.min_x = min_x
        self.max_x = max_x
        self.min_y = min_y
        self.max_y = max_y

    def process(self):
        for e, (pos, vel) in self.world.get_components(Position, Velocity):
            pos.x += vel.x
            pos.y += vel.y

            pos.x = max(self.min_x, pos.x)
            pos.y = max(self.min_y, pos.y)
            pos.x = min(self.max_x - pos.w, pos.x)
            pos.y = min(self.max_y - pos.h, pos.y)


# =====================================================================================================================
# GAME LOOP
# =====================================================================================================================
def run():
    pygame.init()
    window = pygame.display.set_mode(RESOLUTION, pygame.SCALED, vsync=1)
    pygame.display.set_caption("Pong!")
    clock = pygame.time.Clock()
    pygame.key.set_repeat(1, 1)

    # World
    world = esper.World()

    # Entities
    player_1 = world.create_entity()
    world.add_component(player_1, Drawable(is_drawable=True))
    world.add_component(player_1, Velocity(x=0.0, y=0.0))
    world.add_component(player_1, Position(x=PLAYER_WIDTH + 32,
                                           y=(RESOLUTION[1] / 2) - (PLAYER_HEIGHT / 2),
                                           w=PLAYER_WIDTH,
                                           h=PLAYER_HEIGHT))

    player_2 = world.create_entity()
    world.add_component(player_2, Drawable(is_drawable=True))
    world.add_component(player_2, Velocity(x=0.0, y=0.0))
    world.add_component(player_2, Position(x=RESOLUTION[0] - (PLAYER_WIDTH * 2) - 32,
                                           y=(RESOLUTION[1] / 2) - (PLAYER_HEIGHT / 2),
                                           w=PLAYER_WIDTH,
                                           h=PLAYER_HEIGHT))

    # Systems
    draw_system = DrawSystem(window=window)
    movement_system = MovementSystem(min_x=16, max_x=RESOLUTION[0] - 16, min_y=16, max_y=RESOLUTION[1] - 16)

    world.add_processor(draw_system)
    world.add_processor(movement_system)

    running = True

    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_a:
                    world.component_for_entity(player_1, Velocity).y = -PLAYER_SPD

                if event.key == pygame.K_d:
                    world.component_for_entity(player_1, Velocity).y = PLAYER_SPD

                if event.key == pygame.K_LEFT:
                    world.component_for_entity(player_2, Velocity).y = -PLAYER_SPD

                if event.key == pygame.K_RIGHT:
                    world.component_for_entity(player_2, Velocity).y = PLAYER_SPD

            elif event.type == pygame.KEYUP:
                if event.key == pygame.K_a or pygame.K_d:
                    world.component_for_entity(player_1, Velocity).y = 0.0

                if event.key == pygame.K_LEFT or pygame.K_RIGHT:
                    world.component_for_entity(player_2, Velocity).y = 0.0

        world.process()
        clock.tick(FPS)


if __name__ == "__main__":
    run()
    pygame.quit()
