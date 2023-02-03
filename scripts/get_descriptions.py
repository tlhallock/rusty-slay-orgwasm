import os
import re
from tree_sitter import Language, Parser


HERO_RE = re.compile("HeroAbilityType::([^ ]*) => CardSpec {")
MONSTER_RE = re.compile("Monster::([^ ]*) => CardSpec {")
PARTY_LEADER = re.compile("HeroType::([^ ]*) => CardSpec {")
ITEM = re.compile("Item::([^ ]*) => CardSpec {")
SPELL = re.compile("MagicSpell::([^ ]*) => CardSpec {")

DESCRIPTION_RE = re.compile('description: "([^"]*)".to_string\\(\\)')
LABEL_RE = re.compile('label: "([^"]*)".to_string\\(\\)')
IMAGE = re.compile('image_path: "([^"]*)".to_string\\(\\)')
IMAGE = re.compile('condition: Condition::ge\\(([0-9]+)\\),')


def print_descriptions(language, file_path):
	with open(file_path, "r") as fp:
		label = None
		description = None

		for line in fp:
			m = HERO_RE.search(line)
			if m:
				label = m.group(1)

			m = IMAGE.search(line)
			if m:
				description = m.group(1)

				if label and description:
					# print(f'HeroAbilityType::{label} => "{description}",')
					print(f'HeroAbilityType::{label} => Condition::ge({description}),')
					label = None
					description = None




	"""
	with open(file_path, "r") as fp:
		contents = "".join(fp.readlines())

	parser = Parser()
	parser.set_language(language)
	tree = parser.parse(bytes(contents, "utf8"))
	# for element in tree:
	# 	print(element)
	arms = tree.root_node.children[-1].children[-1].children[1].children[-1].children[1].children[0].children[-1].children
	heros = arms[1]
	import pdb; pdb.set_trace()
	for node in heros.children[2].children[-1].children:
		print(node)
	print(tree)
	"""


def main():
	Language.build_library(
		'build/my-languages.so',
		['scripts/tree-sitter-rust'],
	)
	language = Language('build/my-languages.so', 'rust')
	print_descriptions(language, "src/slay/specs/initialization.rs")


if __name__ == "__main__":
	main()

