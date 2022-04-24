export module Sandcore.World.Block;

export import Sandcore.World.Block.Identification;

export class Block
{
public:

	Block(BlockIdentification identification = BlockIdentification::vacuum);

	BlockIdentification& getId();

private:

	BlockIdentification identification;
};